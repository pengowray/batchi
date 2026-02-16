/// Simple single-pole IIR low-pass filter (first-order exponential moving average).
///
/// Transfer function: y[n] = alpha * x[n] + (1 - alpha) * y[n-1]
/// where alpha = dt / (rc + dt), rc = 1 / (2 * PI * cutoff_hz)
///
/// For production use, upgrade to a higher-order Butterworth or Chebyshev filter
/// for sharper rolloff.
pub fn lowpass_filter(samples: &[f32], cutoff_hz: f64, sample_rate: u32) -> Vec<f32> {
    if samples.is_empty() {
        return Vec::new();
    }

    let dt = 1.0 / sample_rate as f64;
    let rc = 1.0 / (2.0 * std::f64::consts::PI * cutoff_hz);
    let alpha = (dt / (rc + dt)) as f32;

    let mut output = Vec::with_capacity(samples.len());
    let mut prev = samples[0];
    output.push(prev);

    for &sample in &samples[1..] {
        let filtered = alpha * sample + (1.0 - alpha) * prev;
        output.push(filtered);
        prev = filtered;
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lowpass_attenuates_high_frequency() {
        let sample_rate = 192_000u32;
        let num_samples = 19200; // 100ms

        // Generate a 50 kHz signal
        let high: Vec<f32> = (0..num_samples)
            .map(|i| {
                let t = i as f64 / sample_rate as f64;
                (2.0 * std::f64::consts::PI * 50_000.0 * t).sin() as f32
            })
            .collect();

        let filtered = lowpass_filter(&high, 10_000.0, sample_rate);

        let rms_in: f64 =
            (high.iter().map(|s| (*s as f64).powi(2)).sum::<f64>() / high.len() as f64).sqrt();
        let rms_out: f64 = (filtered.iter().map(|s| (*s as f64).powi(2)).sum::<f64>()
            / filtered.len() as f64)
            .sqrt();

        assert!(
            rms_out < rms_in * 0.3,
            "50 kHz should be attenuated by LPF at 10 kHz: in={rms_in}, out={rms_out}"
        );
    }

    #[test]
    fn test_lowpass_passes_low_frequency() {
        let sample_rate = 192_000u32;
        let num_samples = 19200;

        // Generate a 1 kHz signal (well below 10 kHz cutoff)
        let low: Vec<f32> = (0..num_samples)
            .map(|i| {
                let t = i as f64 / sample_rate as f64;
                (2.0 * std::f64::consts::PI * 1_000.0 * t).sin() as f32
            })
            .collect();

        let filtered = lowpass_filter(&low, 10_000.0, sample_rate);

        let rms_in: f64 =
            (low.iter().map(|s| (*s as f64).powi(2)).sum::<f64>() / low.len() as f64).sqrt();
        let rms_out: f64 = (filtered.iter().map(|s| (*s as f64).powi(2)).sum::<f64>()
            / filtered.len() as f64)
            .sqrt();

        assert!(
            rms_out > rms_in * 0.8,
            "1 kHz should pass through LPF at 10 kHz: in={rms_in}, out={rms_out}"
        );
    }

    #[test]
    fn test_empty_input() {
        let result = lowpass_filter(&[], 1000.0, 44100);
        assert!(result.is_empty());
    }
}
