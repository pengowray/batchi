use crate::dsp::filters::lowpass_filter;

/// Simulate a zero-crossing frequency division bat detector.
///
/// Real FD detectors work by:
/// 1. Bandpass filtering the input to the ultrasonic range (15-150 kHz)
/// 2. Using a Schmitt trigger (hysteresis comparator) to reject noise crossings
/// 3. Dividing the crossing rate by `division_factor`
/// 4. Outputting a short pulse at each divided crossing
///
/// The output amplitude tracks the input envelope so that louder bat calls
/// produce louder clicks, matching the behavior of analog FD detectors.
pub fn zc_divide(samples: &[f32], sample_rate: u32, division_factor: u32) -> Vec<f32> {
    if samples.len() < 2 || division_factor == 0 {
        return vec![0.0; samples.len()];
    }

    // --- Step 1: Bandpass filter to ultrasonic range (15 kHz – Nyquist) ---
    // High-pass at 15 kHz via subtracting lowpass from original.
    // This removes audible-range noise that causes spurious crossings.
    let lp = cascaded_lp(samples, 15_000.0, sample_rate, 4);
    let filtered: Vec<f32> = samples.iter().zip(lp.iter()).map(|(s, l)| s - l).collect();

    // Low-pass at 150 kHz (only matters if sample rate > 300 kHz)
    let nyquist = sample_rate as f64 / 2.0;
    let filtered = if nyquist > 150_000.0 {
        cascaded_lp(&filtered, 150_000.0, sample_rate, 4)
    } else {
        filtered
    };

    // --- Step 2: Compute envelope for adaptive threshold & output amplitude ---
    // Smooth envelope follower (~1ms attack/release)
    let env_samples = (sample_rate as f64 * 0.001) as usize;
    let env_samples = env_samples.max(1);
    let envelope = smooth_envelope(&filtered, env_samples);

    // Schmitt trigger threshold: fixed at a level that rejects noise but
    // catches real bat calls. -40 dBFS (0.01) works for most recordings.
    // Real FD detectors have a fixed comparator threshold too.
    let threshold_high: f32 = 0.01;
    let threshold_low: f32 = 0.005;

    // --- Step 3: Schmitt trigger zero-crossing detection with division ---
    let mut output = vec![0.0f32; samples.len()];
    let mut crossing_count: u32 = 0;
    let mut armed = false; // Schmitt trigger state: signal above threshold
    let mut prev_positive = filtered[0] >= 0.0;

    // Click duration: ~0.15ms — short enough to sound like a click, long enough to be audible
    let click_len = ((sample_rate as f64 * 0.00015) as usize).max(2);

    // Output gain: 0.3 to avoid being too loud
    let output_gain: f32 = 0.3;

    for i in 1..filtered.len() {
        let env = envelope[i];

        // Schmitt trigger: arm when envelope exceeds high threshold,
        // disarm when it drops below low threshold
        if env > threshold_high {
            armed = true;
        } else if env < threshold_low {
            armed = false;
            crossing_count = 0; // Reset counter during silence
        }

        let curr_positive = filtered[i] >= 0.0;
        if armed && prev_positive != curr_positive {
            crossing_count += 1;
            if crossing_count >= division_factor {
                crossing_count = 0;
                // Scale click amplitude by envelope (quieter calls → quieter clicks)
                let amp = (env / threshold_high).min(1.0) * output_gain;
                let end = (i + click_len).min(samples.len());
                for j in i..end {
                    let phase = (j - i) as f64 / click_len as f64 * std::f64::consts::PI;
                    output[j] = phase.sin() as f32 * amp;
                }
            }
        }
        prev_positive = curr_positive;
    }

    // --- Step 4: Lowpass the output to soften clicks slightly ---
    cascaded_lp(&output, 12_000.0, sample_rate, 2)
}

fn cascaded_lp(samples: &[f32], cutoff: f64, sample_rate: u32, passes: usize) -> Vec<f32> {
    let mut result = samples.to_vec();
    for _ in 0..passes {
        result = lowpass_filter(&result, cutoff, sample_rate);
    }
    result
}

/// Simple envelope follower using a sliding maximum with exponential decay.
fn smooth_envelope(samples: &[f32], window: usize) -> Vec<f32> {
    let mut env = vec![0.0f32; samples.len()];
    let attack = 1.0 / window as f32; // fast attack
    let release = 1.0 / (window as f32 * 4.0); // slower release

    let mut current = 0.0f32;
    for (i, &s) in samples.iter().enumerate() {
        let abs = s.abs();
        if abs > current {
            current += attack * (abs - current);
        } else {
            current += release * (abs - current);
        }
        env[i] = current;
    }
    env
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    fn make_sine(freq: f64, sample_rate: u32, duration: f64) -> Vec<f32> {
        let n = (sample_rate as f64 * duration) as usize;
        (0..n)
            .map(|i| {
                let t = i as f64 / sample_rate as f64;
                (2.0 * PI * freq * t).sin() as f32
            })
            .collect()
    }

    #[test]
    fn test_ultrasonic_sine_produces_clicks() {
        let sr = 192_000;
        // Strong 45 kHz signal — well above any noise floor
        let input: Vec<f32> = make_sine(45_000.0, sr, 0.02)
            .iter()
            .map(|s| s * 0.8)
            .collect();
        let output = zc_divide(&input, sr, 10);
        assert_eq!(output.len(), input.len());

        let peak = output.iter().map(|s| s.abs()).fold(0.0f32, f32::max);
        assert!(peak > 0.01, "Should produce audible clicks, peak={peak}");
        assert!(peak < 0.5, "Should not be too loud, peak={peak}");
    }

    #[test]
    fn test_silence_produces_no_output() {
        let input = vec![0.0f32; 19200];
        let output = zc_divide(&input, 192_000, 10);
        let peak = output.iter().map(|s| s.abs()).fold(0.0f32, f32::max);
        assert!(peak < 0.001, "Silence should produce no clicks");
    }

    #[test]
    fn test_low_noise_rejected() {
        // Very quiet signal (noise-level) should be gated out
        let input: Vec<f32> = make_sine(45_000.0, 192_000, 0.02)
            .iter()
            .map(|s| s * 0.001) // ~60 dB below full scale
            .collect();
        let output = zc_divide(&input, 192_000, 10);
        let peak = output.iter().map(|s| s.abs()).fold(0.0f32, f32::max);
        assert!(peak < 0.01, "Noise-level input should be gated, peak={peak}");
    }

    #[test]
    fn test_empty_input() {
        let output = zc_divide(&[], 192_000, 10);
        assert!(output.is_empty());
    }

    #[test]
    fn test_dc_signal_no_clicks() {
        let input = vec![1.0f32; 1000];
        let output = zc_divide(&input, 44100, 10);
        assert!(output.iter().all(|&s| s.abs() < 0.001));
    }
}
