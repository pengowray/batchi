/// Per-bit statistics for a single bit position.
#[derive(Clone, Debug, PartialEq)]
pub struct BitStat {
    pub count: usize,
    pub first_index: Option<usize>,
    pub last_index: Option<usize>,
}

/// Caution flags for a single bit.
#[derive(Clone, Debug, PartialEq)]
pub enum BitCaution {
    /// Bit is only set during the first or last second (fade artifact)
    OnlyInFade,
    /// Bit is always 1 in every sample
    Always1,
    /// Very low usage count for a bit that should be populated
    VeryLowUsage,
}

/// Full bit-depth analysis result.
#[derive(Clone, Debug, PartialEq)]
pub struct BitAnalysis {
    pub bits_per_sample: u16,
    pub is_float: bool,
    pub total_samples: usize,
    pub duration_secs: f64,
    pub bit_stats: Vec<BitStat>,
    pub bit_cautions: Vec<Vec<BitCaution>>,
    pub effective_bits: u16,
    pub summary: String,
    pub warnings: Vec<String>,
}

/// Bit label for display in the grid.
pub fn bit_label(bit_index: usize, bits_per_sample: u16, is_float: bool) -> String {
    let bit_pos = bits_per_sample as usize - 1 - bit_index;
    if is_float && bits_per_sample == 32 {
        match bit_pos {
            31 => "S".into(),
            23..=30 => format!("E{}", bit_pos - 23),
            _ => format!("M{}", bit_pos),
        }
    } else {
        if bit_pos == bits_per_sample as usize - 1 {
            "S".into()
        } else {
            format!("{}", bit_pos)
        }
    }
}

/// Analyze bit usage across all samples.
pub fn analyze_bits(
    samples: &[f32],
    bits_per_sample: u16,
    is_float: bool,
    duration_secs: f64,
) -> BitAnalysis {
    let n_bits = bits_per_sample as usize;
    let total = samples.len();

    let mut counts = vec![0usize; n_bits];
    let mut first = vec![None::<usize>; n_bits];
    let mut last = vec![None::<usize>; n_bits];

    if is_float && bits_per_sample == 32 {
        analyze_float_bits(samples, &mut counts, &mut first, &mut last);
    } else {
        analyze_int_bits(samples, bits_per_sample, &mut counts, &mut first, &mut last);
    }

    let bit_stats: Vec<BitStat> = (0..n_bits)
        .map(|i| BitStat {
            count: counts[i],
            first_index: first[i],
            last_index: last[i],
        })
        .collect();

    let effective_bits = detect_effective_bits(&bit_stats, bits_per_sample, is_float);

    let bit_cautions = compute_cautions(
        &bit_stats,
        bits_per_sample,
        is_float,
        total,
        duration_secs,
    );

    let mut warnings = Vec::new();
    if total < 1000 {
        warnings.push("Very low sample count — analysis may be unreliable".into());
    }

    let summary = make_summary(bits_per_sample, is_float, effective_bits, total);

    BitAnalysis {
        bits_per_sample,
        is_float,
        total_samples: total,
        duration_secs,
        bit_stats,
        bit_cautions,
        effective_bits,
        summary,
        warnings,
    }
}

fn analyze_int_bits(
    samples: &[f32],
    bits_per_sample: u16,
    counts: &mut [usize],
    first: &mut [Option<usize>],
    last: &mut [Option<usize>],
) {
    let n_bits = bits_per_sample as usize;
    let max_val = (1u32 << (bits_per_sample - 1)) as f64;
    let mask_bits = bits_per_sample as u32;

    for (idx, &s) in samples.iter().enumerate() {
        let int_val = (s as f64 * max_val).round() as i32;
        // Reinterpret as unsigned for bit inspection
        let bits = int_val as u32;
        for b in 0..n_bits {
            // bit_index 0 = MSB (sign bit), bit_index n-1 = LSB
            let bit_pos = mask_bits as usize - 1 - b;
            if bits & (1 << bit_pos) != 0 {
                counts[b] += 1;
                if first[b].is_none() {
                    first[b] = Some(idx);
                }
                last[b] = Some(idx);
            }
        }
    }
}

fn analyze_float_bits(
    samples: &[f32],
    counts: &mut [usize],
    first: &mut [Option<usize>],
    last: &mut [Option<usize>],
) {
    for (idx, &s) in samples.iter().enumerate() {
        let bits = s.to_bits();
        for b in 0..32usize {
            // bit_index 0 = bit 31 (sign), bit_index 31 = bit 0 (LSB mantissa)
            let bit_pos = 31 - b;
            if bits & (1 << bit_pos) != 0 {
                counts[b] += 1;
                if first[b].is_none() {
                    first[b] = Some(idx);
                }
                last[b] = Some(idx);
            }
        }
    }
}

fn detect_effective_bits(
    stats: &[BitStat],
    bits_per_sample: u16,
    is_float: bool,
) -> u16 {
    if is_float {
        // For float, count how many trailing mantissa bits are unused
        // Mantissa bits are at indices 9..32 (bit_index 9 = M22, bit_index 31 = M0)
        let mantissa_start = 9; // first mantissa bit index
        let mut unused_trailing = 0u16;
        for b in (mantissa_start..32).rev() {
            if stats[b].count == 0 {
                unused_trailing += 1;
            } else {
                break;
            }
        }
        let mantissa_bits_used = 23 - unused_trailing;
        // Approximate equivalent integer bits: 1 (sign) + mantissa_bits_used
        // This is a rough heuristic
        (1 + mantissa_bits_used).min(bits_per_sample)
    } else {
        // For integer, count unused LSBs
        let n = bits_per_sample as usize;
        let mut unused_lsb = 0u16;
        for b in (0..n).rev() {
            // b goes from 0 (MSB) to n-1 (LSB); we scan from LSB upward
            if stats[b].count == 0 {
                unused_lsb += 1;
            } else {
                break;
            }
        }
        bits_per_sample - unused_lsb
    }
}

fn compute_cautions(
    stats: &[BitStat],
    bits_per_sample: u16,
    is_float: bool,
    total: usize,
    duration_secs: f64,
) -> Vec<Vec<BitCaution>> {
    let n = bits_per_sample as usize;
    let sample_rate_approx = if duration_secs > 0.0 {
        total as f64 / duration_secs
    } else {
        0.0
    };
    let one_sec_samples = sample_rate_approx as usize;
    let track_fades = duration_secs >= 3.0 && one_sec_samples > 0;

    (0..n)
        .map(|b| {
            let stat = &stats[b];
            let mut cautions = Vec::new();

            if stat.count == 0 {
                return cautions;
            }

            // Always 1
            if stat.count == total {
                cautions.push(BitCaution::Always1);
            }

            // Only used in fade (first/last second)
            if track_fades {
                if let (Some(fi), Some(li)) = (stat.first_index, stat.last_index) {
                    let in_first_sec = fi < one_sec_samples;
                    let in_last_sec = li >= total.saturating_sub(one_sec_samples);
                    let only_edges = in_first_sec && in_last_sec
                        && li.saturating_sub(fi) < 2 * one_sec_samples
                        && stat.count < one_sec_samples * 2;
                    if only_edges && stat.count < total / 2 {
                        cautions.push(BitCaution::OnlyInFade);
                    }
                }
            }

            // Very low usage for bits that should be populated
            if !is_float {
                // For lower value bits (not sign bit) or sign bit with very few uses
                let is_sign = b == 0;
                let threshold = if is_sign { total / 1000 } else { total / 10000 };
                if stat.count > 0 && stat.count <= threshold.max(1) && total > 1000 {
                    cautions.push(BitCaution::VeryLowUsage);
                }
            }

            cautions
        })
        .collect()
}

fn make_summary(
    bits_per_sample: u16,
    is_float: bool,
    effective_bits: u16,
    total_samples: usize,
) -> String {
    if total_samples == 0 {
        return "No samples".into();
    }

    if is_float {
        if effective_bits < 24 {
            format!(
                "~{}-bit precision in 32-bit float",
                effective_bits
            )
        } else {
            "32-bit float".into()
        }
    } else if effective_bits < bits_per_sample {
        format!(
            "{}-bit recording stored as {}-bit",
            effective_bits, bits_per_sample
        )
    } else {
        format!("{}-bit", bits_per_sample)
    }
}

/// Determine if a bit at the given index is "expected to be used" (shown red if unused).
pub fn is_expected_used(
    bit_index: usize,
    bits_per_sample: u16,
    is_float: bool,
    effective_bits: u16,
) -> bool {
    if is_float && bits_per_sample == 32 {
        match bit_index {
            0 => true,    // sign bit should be used (unless all positive)
            1..=8 => true, // exponent bits should generally be used
            _ => {
                // Mantissa: only upper bits expected
                let mantissa_idx = bit_index - 9; // 0 = M22, 22 = M0
                let expected_mantissa = if effective_bits > 1 {
                    (effective_bits - 1) as usize
                } else {
                    0
                };
                mantissa_idx < expected_mantissa
            }
        }
    } else {
        // Integer: bits from sign down to effective_bits boundary are expected
        // bit_index 0 = MSB (sign), bit_index n-1 = LSB
        let bit_pos = bits_per_sample as usize - 1 - bit_index;
        bit_pos >= (bits_per_sample - effective_bits) as usize
    }
}
