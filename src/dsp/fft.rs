use crate::types::{AudioData, SpectrogramColumn, SpectrogramData};
use realfft::RealFftPlanner;

/// Compute a spectrogram from audio data using a Short-Time Fourier Transform (STFT).
///
/// Uses a Hann window for spectral leakage reduction.
pub fn compute_spectrogram(
    audio: &AudioData,
    fft_size: usize,
    hop_size: usize,
) -> SpectrogramData {
    let mut planner = RealFftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(fft_size);

    let mut columns = Vec::new();

    // Hann window
    let window: Vec<f32> = (0..fft_size)
        .map(|i| {
            0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / (fft_size - 1) as f32).cos())
        })
        .collect();

    let mut pos = 0;
    while pos + fft_size <= audio.samples.len() {
        let mut input: Vec<f32> = audio.samples[pos..pos + fft_size]
            .iter()
            .zip(window.iter())
            .map(|(&s, &w)| s * w)
            .collect();

        let mut spectrum = fft.make_output_vec();
        fft.process(&mut input, &mut spectrum).expect("FFT failed");

        let magnitudes: Vec<f32> = spectrum.iter().map(|c| c.norm()).collect();

        let time_offset = pos as f64 / audio.sample_rate as f64;
        columns.push(SpectrogramColumn {
            magnitudes,
            time_offset,
        });

        pos += hop_size;
    }

    let freq_resolution = audio.sample_rate as f64 / fft_size as f64;
    let time_resolution = hop_size as f64 / audio.sample_rate as f64;
    let max_freq = audio.sample_rate as f64 / 2.0;

    SpectrogramData {
        columns,
        freq_resolution,
        time_resolution,
        max_freq,
        sample_rate: audio.sample_rate,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::AudioData;

    #[test]
    fn test_spectrogram_basic() {
        let sample_rate = 44100u32;
        let freq = 1000.0f64;
        let num_samples = 4096;

        let samples: Vec<f32> = (0..num_samples)
            .map(|i| {
                let t = i as f64 / sample_rate as f64;
                (2.0 * std::f64::consts::PI * freq * t).sin() as f32
            })
            .collect();

        let audio = AudioData {
            samples,
            sample_rate,
            channels: 1,
            duration_secs: num_samples as f64 / sample_rate as f64,
        };

        let result = compute_spectrogram(&audio, 1024, 512);
        assert!(!result.columns.is_empty());
        assert_eq!(result.sample_rate, sample_rate);

        // The peak bin should be near 1000 Hz
        let col = &result.columns[1]; // skip first column (edge effects)
        let peak_bin = col
            .magnitudes
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;
        let peak_freq = peak_bin as f64 * result.freq_resolution;
        let error = (peak_freq - freq).abs();
        assert!(
            error < result.freq_resolution * 2.0,
            "Peak at {peak_freq} Hz, expected ~{freq} Hz"
        );
    }
}
