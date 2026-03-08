use web_sys::CanvasRenderingContext2d;

// ── Time scale ────────────────────────────────────────────────────────────

/// Nice 1-2-5 progression of tick intervals in seconds, from 0.1 ms to 10 min.
const TICK_INTERVALS: &[f64] = &[
    0.0001, 0.0002, 0.0005,         // sub-ms
    0.001, 0.002, 0.005,             // 1–5 ms
    0.01, 0.02, 0.05,               // 10–50 ms
    0.1, 0.2, 0.5,                  // 100–500 ms
    1.0, 2.0, 5.0,                  // 1–5 s
    10.0, 30.0, 60.0,               // 10 s – 1 min
    120.0, 300.0, 600.0,            // 2–10 min
];

/// Configuration for clock-time display on the timeline.
#[derive(Clone, Copy, Debug)]
pub struct ClockTimeConfig {
    /// Recording start time as milliseconds since Unix epoch.
    pub recording_start_epoch_ms: f64,
}

// ── Formatting helpers ───────────────────────────────────────────────────

/// Format a time value as a compact, human-readable label.
///
/// The format is chosen based on the absolute time value, not just the tick
/// interval, so we never produce unreadable labels like "50050ms".
fn format_time_label(seconds: f64, interval: f64) -> String {
    let abs = seconds.abs();

    // Sub-second values: use milliseconds
    if abs < 1.0 {
        let ms = seconds * 1000.0;
        if interval < 0.001 {
            return format!("{:.1}ms", ms);
        } else if interval < 0.01 {
            return format!("{:.1}ms", ms);
        } else {
            return format!("{:.0}ms", ms);
        }
    }

    // 1s to 60s: show as seconds with precision matching the interval
    if abs < 60.0 {
        return format_seconds(seconds, interval);
    }

    // 60s to 3600s: m:ss notation
    if abs < 3600.0 {
        return format_minutes_seconds(seconds, interval);
    }

    // 3600s+: h:mm:ss
    format_hours_minutes_seconds(seconds, interval)
}

fn format_seconds(seconds: f64, interval: f64) -> String {
    if interval >= 1.0 {
        format!("{:.0}s", seconds)
    } else if interval >= 0.1 {
        format!("{:.1}s", seconds)
    } else if interval >= 0.01 {
        format!("{:.2}s", seconds)
    } else {
        format!("{:.3}s", seconds)
    }
}

fn format_minutes_seconds(seconds: f64, interval: f64) -> String {
    let sign = if seconds < 0.0 { "-" } else { "" };
    let abs = seconds.abs();
    let mins = (abs / 60.0).floor() as u32;
    let secs = abs - mins as f64 * 60.0;
    if interval >= 1.0 {
        if secs.round() as u32 == 0 {
            format!("{}{}m", sign, mins)
        } else {
            format!("{}{}:{:02.0}", sign, mins, secs)
        }
    } else if interval >= 0.1 {
        format!("{}{}:{:04.1}", sign, mins, secs)
    } else {
        format!("{}{}:{:06.3}", sign, mins, secs)
    }
}

fn format_hours_minutes_seconds(seconds: f64, interval: f64) -> String {
    let sign = if seconds < 0.0 { "-" } else { "" };
    let abs = seconds.abs();
    let hours = (abs / 3600.0).floor() as u32;
    let rem = abs - hours as f64 * 3600.0;
    let mins = (rem / 60.0).floor() as u32;
    let secs = rem - mins as f64 * 60.0;
    if interval >= 1.0 {
        format!("{}{}:{:02}:{:02.0}", sign, hours, mins, secs)
    } else {
        format!("{}{}:{:02}:{:04.1}", sign, hours, mins, secs)
    }
}

/// Format a relative time offset as "+50ms", "+0.2s", etc.
fn format_relative_label(offset: f64, interval: f64) -> String {
    let abs = offset.abs();
    if abs < 0.0005 {
        return String::new();
    }
    let sign = if offset >= 0.0 { "+" } else { "\u{2212}" }; // − (minus sign)

    if abs < 1.0 {
        let ms = abs * 1000.0;
        if interval < 0.01 {
            format!("{}{:.1}ms", sign, ms)
        } else {
            format!("{}{:.0}ms", sign, ms)
        }
    } else if interval >= 1.0 {
        format!("{}{:.0}s", sign, abs)
    } else if interval >= 0.1 {
        format!("{}{:.1}s", sign, abs)
    } else {
        format!("{}{:.2}s", sign, abs)
    }
}

/// Format a clock time (epoch ms + file offset) as HH:MM:SS with sub-second precision.
fn format_clock_time(epoch_ms: f64, file_offset_secs: f64, interval: f64) -> String {
    let total_ms = epoch_ms + file_offset_secs * 1000.0;
    let date = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(total_ms));
    let h = date.get_hours();
    let m = date.get_minutes();
    let s = date.get_seconds();
    let ms = date.get_milliseconds();

    if interval >= 1.0 {
        format!("{:02}:{:02}:{:02}", h, m, s)
    } else if interval >= 0.1 {
        format!("{:02}:{:02}:{:02}.{}", h, m, s, ms / 100)
    } else if interval >= 0.01 {
        format!("{:02}:{:02}:{:02}.{:02}", h, m, s, ms / 10)
    } else {
        format!("{:02}:{:02}:{:02}.{:03}", h, m, s, ms)
    }
}

/// Format a full datetime string for tooltip display, including the source.
///
/// `source` should describe where the timestamp came from, e.g.
/// "GUANO Timestamp" or "File modified date".
pub fn format_clock_time_full(epoch_ms: f64, file_offset_secs: f64, source: &str) -> String {
    let total_ms = epoch_ms + file_offset_secs * 1000.0;
    let date = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(total_ms));
    let tz_offset_min = date.get_timezone_offset(); // positive = west of UTC
    let tz_h = -(tz_offset_min / 60.0).trunc() as i32;
    let tz_m = (tz_offset_min.abs() % 60.0) as u32;
    let tz_sign = if tz_h >= 0 { "+" } else { "-" };

    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03} UTC{}{:02}:{:02} ({})",
        date.get_full_year(),
        date.get_month() + 1,
        date.get_date(),
        date.get_hours(),
        date.get_minutes(),
        date.get_seconds(),
        date.get_milliseconds(),
        tz_sign,
        tz_h.unsigned_abs(),
        tz_m,
        source,
    )
}

// ── Key marker logic ─────────────────────────────────────────────────────

/// Determine a "key interval" — a larger, rounder interval for primary labels.
/// When zoomed in with sub-second ticks deep into a file, key markers appear at
/// round second/minute boundaries while intermediate ticks show "+Xms" offsets.
fn key_interval_for(interval: f64) -> f64 {
    if interval >= 1.0 {
        return interval;
    }
    let min_key = (interval * 5.0).max(1.0);
    TICK_INTERVALS
        .iter()
        .copied()
        .find(|&i| i >= min_key)
        .unwrap_or(interval)
}

/// Returns true if `t` falls on a key interval boundary.
fn is_key_tick(t: f64, key_interval: f64) -> bool {
    let remainder = (t % key_interval).abs();
    remainder < key_interval * 0.001 || (key_interval - remainder) < key_interval * 0.001
}

// ── Drawing ──────────────────────────────────────────────────────────────

/// Draw time tick marks and labels along the bottom of a canvas.
///
/// When `show_clock_time` is true and a `ClockTimeConfig` is provided, labels
/// display wall-clock time (HH:MM:SS) instead of file-relative time.
///
/// When zoomed in with sub-second tick intervals and the view is deep into the
/// file (>10 s), a key-marker system kicks in: "round" boundaries (e.g. every
/// second) get prominent absolute labels while intermediate ticks show compact
/// relative offsets like "+50ms".
pub fn draw_time_markers(
    ctx: &CanvasRenderingContext2d,
    scroll_offset: f64,
    visible_time: f64,
    canvas_width: f64,
    canvas_height: f64,
    duration: f64,
    clock: Option<ClockTimeConfig>,
    show_clock_time: bool,
) {
    if visible_time <= 0.0 || canvas_width <= 0.0 {
        return;
    }

    let px_per_sec = canvas_width / visible_time;

    // Pick the smallest nice interval that keeps labels ≥100 px apart
    let min_interval = 100.0 / px_per_sec;
    let interval = TICK_INTERVALS
        .iter()
        .copied()
        .find(|&i| i >= min_interval)
        .unwrap_or(*TICK_INTERVALS.last().unwrap());

    let end_time = (scroll_offset + visible_time).min(duration);

    // Key marker system: when sub-second ticks are deep into the file,
    // use primary/secondary label hierarchy
    let key_interval = key_interval_for(interval);
    let first_tick = (scroll_offset / interval).ceil() * interval;
    let use_relative = interval < 1.0 && first_tick > 10.0;
    let use_clock = show_clock_time && clock.is_some();

    // ── Minor ticks (no labels) ──
    let minor_interval = interval / 5.0;
    let minor_px = minor_interval * px_per_sec;
    if minor_px >= 4.0 {
        let first_minor = (scroll_offset / minor_interval).ceil() * minor_interval;
        ctx.set_stroke_style_str("rgba(255,255,255,0.15)");
        ctx.set_line_width(1.0);
        let mut t = first_minor;
        while t <= end_time + minor_interval * 0.5 {
            // Skip major-tick positions
            if ((t / interval).round() * interval - t).abs() < minor_interval * 0.01 {
                t += minor_interval;
                continue;
            }
            let x = (t - scroll_offset) * px_per_sec;
            if x >= 0.0 && x <= canvas_width {
                ctx.begin_path();
                ctx.move_to(x, canvas_height - 6.0);
                ctx.line_to(x, canvas_height);
                ctx.stroke();
            }
            t += minor_interval;
        }
    }

    // ── Major ticks + labels ──
    let tick_h = 12.0;
    let key_tick_h = 16.0;
    ctx.set_text_baseline("bottom");

    let mut t = first_tick;
    while t <= end_time + interval * 0.01 {
        let x = (t - scroll_offset) * px_per_sec;
        if x >= 0.0 && x <= canvas_width {
            let is_key = !use_relative || is_key_tick(t, key_interval);
            let current_tick_h = if use_relative && is_key { key_tick_h } else { tick_h };

            // Bottom tick
            let tick_alpha = if is_key { "0.5" } else { "0.3" };
            ctx.set_stroke_style_str(&format!("rgba(255,255,255,{})", tick_alpha));
            ctx.set_line_width(1.0);
            ctx.begin_path();
            ctx.move_to(x, canvas_height - current_tick_h);
            ctx.line_to(x, canvas_height);
            ctx.stroke();

            // Subtle top tick
            ctx.set_stroke_style_str("rgba(255,255,255,0.10)");
            ctx.begin_path();
            ctx.move_to(x, 0.0);
            ctx.line_to(x, 4.0);
            ctx.stroke();

            // Label
            let label = if use_clock {
                let clk = clock.unwrap();
                format_clock_time(clk.recording_start_epoch_ms, t, interval)
            } else if use_relative && !is_key {
                let nearest_key = (t / key_interval).round() * key_interval;
                format_relative_label(t - nearest_key, interval)
            } else {
                format_time_label(t, interval)
            };

            if label.is_empty() {
                t += interval;
                continue;
            }

            let font = if use_relative && is_key {
                "bold 10px sans-serif"
            } else {
                "10px sans-serif"
            };
            ctx.set_font(font);

            if let Ok(metrics) = ctx.measure_text(&label) {
                let tw = metrics.width();
                let lx = x + 3.0;
                if lx + tw < canvas_width - 2.0 {
                    // Dark background for readability
                    ctx.set_fill_style_str("rgba(0,0,0,0.6)");
                    ctx.fill_rect(
                        lx - 1.0,
                        canvas_height - current_tick_h - 12.0,
                        tw + 2.0,
                        12.0,
                    );
                    // Text with emphasis for key markers
                    let text_alpha = if use_relative && is_key { "0.9" } else { "0.7" };
                    ctx.set_fill_style_str(&format!("rgba(255,255,255,{})", text_alpha));
                    let _ = ctx.fill_text(&label, lx, canvas_height - current_tick_h - 1.0);
                }
            }
        }
        t += interval;
    }

    ctx.set_text_baseline("alphabetic"); // reset
    ctx.set_font("10px sans-serif"); // reset
}
