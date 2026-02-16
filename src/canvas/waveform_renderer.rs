use web_sys::CanvasRenderingContext2d;

/// Draw waveform on a canvas context.
/// Uses min/max envelope at low zoom, individual samples at high zoom.
pub fn draw_waveform(
    ctx: &CanvasRenderingContext2d,
    samples: &[f32],
    sample_rate: u32,
    scroll_offset: f64,
    zoom: f64,
    time_resolution: f64,
    canvas_width: f64,
    canvas_height: f64,
    selection: Option<(f64, f64)>,
) {
    // Clear
    ctx.set_fill_style_str("#0a0a0a");
    ctx.fill_rect(0.0, 0.0, canvas_width, canvas_height);

    if samples.is_empty() {
        return;
    }

    let duration = samples.len() as f64 / sample_rate as f64;
    let mid_y = canvas_height / 2.0;

    // Pixels per second — match spectrogram's viewport calculation
    // In the spectrogram, visible_cols = canvas_width / zoom (in spectrogram columns)
    // Each column = time_resolution seconds
    // So visible_time = (canvas_width / zoom) * time_resolution
    let visible_time = (canvas_width / zoom) * time_resolution;
    let start_time = scroll_offset.max(0.0).min((duration - visible_time).max(0.0));
    let px_per_sec = canvas_width / visible_time;

    // Draw selection highlight
    if let Some((sel_start, sel_end)) = selection {
        let x0 = ((sel_start - start_time) * px_per_sec).max(0.0);
        let x1 = ((sel_end - start_time) * px_per_sec).min(canvas_width);
        if x1 > x0 {
            ctx.set_fill_style_str("rgba(50, 120, 200, 0.2)");
            ctx.fill_rect(x0, 0.0, x1 - x0, canvas_height);
        }
    }

    // Draw center line
    ctx.set_stroke_style_str("#333");
    ctx.set_line_width(1.0);
    ctx.begin_path();
    ctx.move_to(0.0, mid_y);
    ctx.line_to(canvas_width, mid_y);
    ctx.stroke();

    // Draw waveform
    ctx.set_stroke_style_str("#6a6");
    ctx.set_line_width(1.0);

    let samples_per_pixel = (visible_time * sample_rate as f64) / canvas_width;

    if samples_per_pixel <= 2.0 {
        // High zoom: draw individual samples as connected lines
        ctx.begin_path();
        let mut first = true;
        for px in 0..(canvas_width as usize) {
            let t = start_time + (px as f64 / px_per_sec);
            let idx = (t * sample_rate as f64) as usize;
            if idx >= samples.len() {
                break;
            }
            let y = mid_y - (samples[idx] as f64 * mid_y * 0.9);
            if first {
                ctx.move_to(px as f64, y);
                first = false;
            } else {
                ctx.line_to(px as f64, y);
            }
        }
        ctx.stroke();
    } else {
        // Low zoom: draw min/max envelope per pixel column
        for px in 0..(canvas_width as usize) {
            let t0 = start_time + (px as f64 / px_per_sec);
            let t1 = start_time + ((px as f64 + 1.0) / px_per_sec);
            let i0 = ((t0 * sample_rate as f64) as usize).min(samples.len());
            let i1 = ((t1 * sample_rate as f64) as usize).min(samples.len());

            if i0 >= i1 || i0 >= samples.len() {
                break;
            }

            let mut min_val = f32::MAX;
            let mut max_val = f32::MIN;
            for &s in &samples[i0..i1] {
                if s < min_val { min_val = s; }
                if s > max_val { max_val = s; }
            }

            let y_min = mid_y - (max_val as f64 * mid_y * 0.9);
            let y_max = mid_y - (min_val as f64 * mid_y * 0.9);

            ctx.begin_path();
            ctx.move_to(px as f64, y_min);
            ctx.line_to(px as f64, y_max);
            ctx.stroke();
        }
    }
}
