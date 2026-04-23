// SPDX-License-Identifier: GPL-3.0-only OR MIT OR Apache-2.0
//! Resonate — a bank of independent complex resonators for spectral analysis.
//!
//! Each resonator is a phasor-driven exponential moving average (EMA):
//!
//! ```text
//!   z_k[n] = (1 - alpha_k) * x[n] * e^(-j·2π·f_k·n / sr) + alpha_k * z_k[n-1]
//! ```
//!
//! where `alpha_k = exp(-2π · bandwidth_k / sr)` controls the per-bin
//! integration time. Output at a given sample is `|z_k|`.
//!
//! Unlike STFT, there is no windowing or buffering — each resonator updates
//! every sample, giving low-latency, per-bin time/frequency tradeoff.
//!
//! Based on Alexandre François's Resonate algorithm:
//! - <https://alexandrefrancois.org/Resonate/>
//! - <https://github.com/alexandrefrancois/noFFT> (C++ reference)
//! - <https://github.com/jhartquist/resonators> (Rust reference)
//!
//! # Layout
//!
//! For compatibility with the existing spectrogram pipeline, this module uses
//! a **linear** frequency layout matching the STFT:
//!   `num_bins = fft_size / 2 + 1`, covering 0..Nyquist with
//!   `f_k = k · (sr/2) / (num_bins - 1)`.
//!
//! This lets the output `SpectrogramColumn`s plug directly into tile caches,
//! live waterfalls, row→freq mapping, and overlays without any special cases.
//!
//! # Warm-up
//!
//! Resonator state is stateful (EMA carries history). Each call starts from
//! zero. For tile-based computation, the caller should pre-pad `samples` with
//! ~5τ additional samples (τ = 1/(2π·bandwidth)) before `col_start * hop_size`
//! so the first emitted column reflects a converged EMA. This module does not
//! manage its own pre-pad — the caller passes raw samples and a `col_start`
//! offset that skips the warm-up columns.

use crate::types::SpectrogramColumn;

/// Recommended warm-up samples for a given bandwidth.
///
/// Returns ≈5τ samples, where τ = 1/(2π·bandwidth) is the EMA time constant.
/// At 5τ the EMA has converged to within ~1% of steady state.
pub fn warmup_samples(sample_rate: u32, bandwidth_hz: f32) -> usize {
    let bw = bandwidth_hz.max(1.0);
    let tau_secs = 1.0 / (std::f32::consts::TAU * bw);
    (5.0 * tau_secs * sample_rate as f32).ceil().max(256.0) as usize
}

/// Compute resonator columns over a slice of audio samples.
///
/// Parameters mirror `dsp::fft::compute_stft_columns`:
/// - `fft_size` determines `num_bins = fft_size/2 + 1` (frequency resolution).
/// - `hop_size` is the output column interval in samples.
/// - `col_start`/`col_count` select which columns to emit (0-based, counted
///   from sample 0). The resonator still processes samples 0..col_end*hop_size
///   to build up state, so the caller should generally pass `col_start = 0`
///   on a pre-padded sample slice and discard warm-up columns afterwards.
///
/// `bandwidth_hz` sets per-bin EMA bandwidth. Typical range 50..2000 Hz:
/// - Smaller ⇒ sharper frequency bins, slower temporal response.
/// - Larger ⇒ faster response, wider bins.
///
/// Output magnitudes are scaled to roughly match STFT magnitude scale so
/// existing gain / floor_db settings produce similar on-screen brightness.
pub fn compute_resonator_columns(
    samples: &[f32],
    sample_rate: u32,
    fft_size: usize,
    hop_size: usize,
    col_start: usize,
    col_count: usize,
    bandwidth_hz: f32,
) -> Vec<SpectrogramColumn> {
    let num_bins = fft_size / 2 + 1;
    if samples.is_empty() || num_bins == 0 || col_count == 0 || hop_size == 0 {
        return vec![];
    }

    let sr_f = sample_rate as f32;
    let nyq = sr_f * 0.5;
    let denom = (num_bins - 1).max(1) as f32;
    let two_pi = std::f32::consts::TAU;

    // Clamp bandwidth to a stable range.
    let bw = bandwidth_hz.clamp(1.0, nyq);
    let alpha = (-two_pi * bw / sr_f).exp();
    let one_minus_alpha = 1.0 - alpha;

    // Pre-compute per-bin phase step (cos, sin).
    let mut step_cos = vec![0.0f32; num_bins];
    let mut step_sin = vec![0.0f32; num_bins];
    for k in 0..num_bins {
        let f_k = k as f32 * nyq / denom;
        let phi = -two_pi * f_k / sr_f;
        step_cos[k] = phi.cos();
        step_sin[k] = phi.sin();
    }

    // Per-bin rotating phasor (starts at 1+0j) and EMA accumulator.
    let mut phasor_re = vec![1.0f32; num_bins];
    let mut phasor_im = vec![0.0f32; num_bins];
    let mut z_re = vec![0.0f32; num_bins];
    let mut z_im = vec![0.0f32; num_bins];

    let col_end = col_start + col_count;
    let max_sample = col_end.saturating_mul(hop_size).min(samples.len());

    // Magnitude scale: roughly match one-sided STFT magnitude with Hann
    // coherent gain, so existing floor_db / gain settings look similar.
    let mag_scale = (fft_size as f32) * 0.5;

    let mut out: Vec<SpectrogramColumn> = Vec::with_capacity(col_count);
    let mut cur_col: usize = 0;
    let mut next_hop: usize = 0;

    for (n, &x) in samples.iter().enumerate().take(max_sample) {
        // Rotate every phasor by one sample, then EMA-accumulate x·phasor.
        for k in 0..num_bins {
            let pr = phasor_re[k];
            let pi = phasor_im[k];
            let new_pr = pr * step_cos[k] - pi * step_sin[k];
            let new_pi = pr * step_sin[k] + pi * step_cos[k];
            phasor_re[k] = new_pr;
            phasor_im[k] = new_pi;

            let demod_re = x * new_pr;
            let demod_im = x * new_pi;

            z_re[k] = alpha * z_re[k] + one_minus_alpha * demod_re;
            z_im[k] = alpha * z_im[k] + one_minus_alpha * demod_im;
        }

        // Emit at each hop boundary.
        if n == next_hop {
            if cur_col >= col_start {
                let mut mags = Vec::with_capacity(num_bins);
                for k in 0..num_bins {
                    let m = (z_re[k] * z_re[k] + z_im[k] * z_im[k]).sqrt() * mag_scale;
                    mags.push(m);
                }
                out.push(SpectrogramColumn {
                    magnitudes: mags,
                    time_offset: n as f64 / sample_rate as f64,
                });
                if out.len() >= col_count {
                    break;
                }
            }
            cur_col += 1;
            next_hop = next_hop.saturating_add(hop_size);
        }

        // Periodically renormalise phasors back to unit length to cancel
        // accumulated floating-point drift over long signals.
        if n != 0 && n % 4096 == 0 {
            for k in 0..num_bins {
                let m2 = phasor_re[k] * phasor_re[k] + phasor_im[k] * phasor_im[k];
                if m2 > 0.0 {
                    let inv = 1.0 / m2.sqrt();
                    phasor_re[k] *= inv;
                    phasor_im[k] *= inv;
                }
            }
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A pure tone should produce a peak at the matching bin.
    #[test]
    fn peak_at_tone_frequency() {
        let sr = 48_000u32;
        let fft_size = 256;
        let hop = 64;
        let num_bins = fft_size / 2 + 1;

        // 6 kHz sine, 1 s long.
        let f = 6_000.0f32;
        let samples: Vec<f32> = (0..sr as usize)
            .map(|i| (std::f32::consts::TAU * f * i as f32 / sr as f32).sin())
            .collect();

        let cols = compute_resonator_columns(&samples, sr, fft_size, hop, 0, 100, 200.0);
        assert!(!cols.is_empty());

        // Look at a column well past warm-up.
        let mid = &cols[cols.len() - 1];
        let (peak_bin, _peak_val) = mid
            .magnitudes
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap();

        // Expected bin: k = f / (nyq/(num_bins-1))
        let nyq = (sr as f32) / 2.0;
        let expected = (f / (nyq / (num_bins - 1) as f32)).round() as usize;
        assert!(
            (peak_bin as isize - expected as isize).abs() <= 1,
            "peak at bin {peak_bin}, expected {expected}"
        );
    }
}
