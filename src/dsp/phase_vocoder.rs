use realfft::RealFftPlanner;
use std::f64::consts::PI;

use super::pitch_shift::{resample_compress, resample_stretch};

const FFT_SIZE: usize = 4096;
const HOP: usize = 1024; // 75% overlap

/// Phase-vocoder pitch shift: time-stretch via STFT then resample to restore duration.
///
/// - `factor > 1.0`: shift DOWN (divide frequencies). E.g. factor=10 shifts 50 kHz → 5 kHz.
/// - `factor < -1.0`: shift UP (multiply frequencies). E.g. factor=-10 shifts 5 Hz → 50 Hz.
/// - `|factor| <= 1.0`: bypass (returns input unchanged).
pub fn phase_vocoder_pitch_shift(samples: &[f32], factor: f64) -> Vec<f32> {
    if samples.is_empty() {
        return Vec::new();
    }

    let abs_factor = factor.abs();
    if abs_factor <= 1.0 {
        return samples.to_vec();
    }

    let shift_up = factor < 0.0;

    // Pitch ratio: how much to multiply frequencies by.
    // shift_down factor=10 → pitch_ratio = 1/10 (lower pitch)
    // shift_up factor=-10 → pitch_ratio = 10 (higher pitch)
    let pitch_ratio = if shift_up { abs_factor } else { 1.0 / abs_factor };

    // Time-stretch ratio: stretch duration by 1/pitch_ratio so that
    // resampling back to original length achieves the desired pitch shift.
    let stretch_ratio = 1.0 / pitch_ratio;

    // Step 1: time-stretch using phase vocoder STFT
    let stretched = phase_vocoder_stretch(samples, stretch_ratio);

    // Step 2: resample back to original length
    let target_len = samples.len();
    let resample_ratio = target_len as f64 / stretched.len() as f64;

    let resampled = if resample_ratio > 1.0 {
        // stretched is shorter → stretch it
        resample_stretch(&stretched, resample_ratio)
    } else if resample_ratio < 1.0 {
        // stretched is longer → compress it
        resample_compress(&stretched, 1.0 / resample_ratio)
    } else {
        stretched
    };

    // Trim or pad to exact target length
    let mut output = resampled;
    output.truncate(target_len);
    while output.len() < target_len {
        output.push(0.0);
    }
    output
}

/// Time-stretch audio by `ratio` using phase vocoder STFT.
/// ratio > 1.0 = longer output (slower), ratio < 1.0 = shorter output (faster).
fn phase_vocoder_stretch(samples: &[f32], ratio: f64) -> Vec<f32> {
    if samples.len() < FFT_SIZE {
        // Too short for STFT — just repeat/truncate
        let out_len = (samples.len() as f64 * ratio).round() as usize;
        return resample_stretch(samples, ratio).into_iter().take(out_len.max(1)).collect();
    }

    let n_bins = FFT_SIZE / 2 + 1;
    let synthesis_hop = (HOP as f64 * ratio).round() as usize;
    let synthesis_hop = synthesis_hop.max(1);

    // Count analysis frames
    let n_frames = (samples.len().saturating_sub(FFT_SIZE)) / HOP + 1;
    if n_frames == 0 {
        return samples.to_vec();
    }

    let out_len = (n_frames - 1) * synthesis_hop + FFT_SIZE;

    // Hann window
    let hann: Vec<f64> = (0..FFT_SIZE)
        .map(|i| {
            let x = PI * i as f64 / FFT_SIZE as f64;
            x.sin().powi(2)
        })
        .collect();

    // Set up FFT
    let mut planner = RealFftPlanner::<f64>::new();
    let fft_forward = planner.plan_fft_forward(FFT_SIZE);
    let fft_inverse = planner.plan_fft_inverse(FFT_SIZE);

    let mut prev_phase = vec![0.0f64; n_bins];
    let mut synth_phase = vec![0.0f64; n_bins];
    let mut output = vec![0.0f64; out_len];
    let mut window_sum = vec![0.0f64; out_len];

    let mut fft_in = vec![0.0f64; FFT_SIZE];
    let mut spectrum = fft_forward.make_output_vec();
    let mut ifft_out = vec![0.0f64; FFT_SIZE];

    for frame in 0..n_frames {
        let offset = frame * HOP;

        // Window the input frame
        for i in 0..FFT_SIZE {
            fft_in[i] = samples[offset + i] as f64 * hann[i];
        }

        // Forward FFT
        fft_forward.process(&mut fft_in, &mut spectrum).unwrap();

        // Phase processing
        for k in 0..n_bins {
            let re = spectrum[k].re;
            let im = spectrum[k].im;
            let mag = (re * re + im * im).sqrt();
            let phase = im.atan2(re);

            // Phase difference from previous frame
            let expected_phase_advance = 2.0 * PI * k as f64 * HOP as f64 / FFT_SIZE as f64;
            let delta_phase = phase - prev_phase[k] - expected_phase_advance;

            // Wrap to [-pi, pi]
            let wrapped = delta_phase - (2.0 * PI) * ((delta_phase + PI) / (2.0 * PI)).floor();

            // True frequency deviation
            let true_freq = expected_phase_advance + wrapped;

            // Accumulate synthesis phase at the new hop rate
            synth_phase[k] += true_freq * (synthesis_hop as f64 / HOP as f64);

            // Reconstruct bin
            spectrum[k].re = mag * synth_phase[k].cos();
            spectrum[k].im = mag * synth_phase[k].sin();

            prev_phase[k] = phase;
        }

        // Inverse FFT
        fft_inverse.process(&mut spectrum, &mut ifft_out).unwrap();

        // Normalize inverse FFT (realfft doesn't normalize)
        let norm = 1.0 / FFT_SIZE as f64;

        // Overlap-add at synthesis position
        let out_offset = frame * synthesis_hop;
        for i in 0..FFT_SIZE {
            if out_offset + i < out_len {
                output[out_offset + i] += ifft_out[i] * norm * hann[i];
                window_sum[out_offset + i] += hann[i] * hann[i];
            }
        }
    }

    // Normalize by window overlap sum
    output
        .iter()
        .zip(window_sum.iter())
        .map(|(&s, &w)| {
            if w > 1e-6 {
                (s / w) as f32
            } else {
                0.0f32
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bypass_small_factor() {
        let input: Vec<f32> = (0..1000).map(|i| (i as f32 * 0.1).sin()).collect();
        assert_eq!(phase_vocoder_pitch_shift(&input, 1.0), input);
        assert_eq!(phase_vocoder_pitch_shift(&input, -1.0), input);
        assert_eq!(phase_vocoder_pitch_shift(&input, 0.5), input);
    }

    #[test]
    fn test_empty_input() {
        assert!(phase_vocoder_pitch_shift(&[], 10.0).is_empty());
    }

    #[test]
    fn test_preserves_length_down() {
        let input: Vec<f32> = (0..8192).map(|i| (i as f32 * 0.01).sin()).collect();
        let output = phase_vocoder_pitch_shift(&input, 10.0);
        assert_eq!(output.len(), input.len());
    }

    #[test]
    fn test_preserves_length_up() {
        let input: Vec<f32> = (0..8192).map(|i| (i as f32 * 0.01).sin()).collect();
        let output = phase_vocoder_pitch_shift(&input, -10.0);
        assert_eq!(output.len(), input.len());
    }

    #[test]
    fn test_stretch_longer() {
        let input: Vec<f32> = (0..8192).map(|i| (i as f32 * 0.01).sin()).collect();
        let stretched = phase_vocoder_stretch(&input, 2.0);
        // Should be roughly 2x as long
        assert!(stretched.len() > input.len());
    }
}
