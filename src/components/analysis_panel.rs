use leptos::prelude::*;
use crate::state::{AppState, CanvasTool};
use crate::annotations::AnnotationKind;

/// Format a selection/annotation's dimensions: duration and optional freq range.
fn format_selection_dims(duration: f64, freq_low: Option<f64>, freq_high: Option<f64>) -> String {
    let dur_str = crate::format_time::format_duration(duration, 3);
    match (freq_low, freq_high) {
        (Some(fl), Some(fh)) => format!(
            "Duration: {}   Freq range: {:.0} – {:.0} kHz",
            dur_str, fl / 1000.0, fh / 1000.0
        ),
        _ => format!("Duration: {}", dur_str),
    }
}

#[component]
pub fn AnalysisPanel() -> impl IntoView {
    let state = expect_context::<AppState>();

    let selection_dims = move || {
        let selection = state.selection.get()?;
        let d = selection.time_end - selection.time_start;
        if d > 0.0001 {
            Some(format_selection_dims(d, selection.freq_low, selection.freq_high))
        } else {
            None
        }
    };

    let annotation_dims = move || {
        let ids = state.selected_annotation_ids.get();
        if ids.is_empty() { return None; }
        let idx = state.current_file_index.get()?;
        let store = state.annotation_store.get();
        let set = store.sets.get(idx)?.as_ref()?;
        // Show dims for single selected annotation
        if ids.len() == 1 {
            let ann = set.annotations.iter().find(|a| a.id == ids[0])?;
            match &ann.kind {
                AnnotationKind::Region(r) => {
                    let d = r.time_end - r.time_start;
                    if d > 0.0001 {
                        Some(format_selection_dims(d, r.freq_low, r.freq_high))
                    } else {
                        None
                    }
                }
                _ => None,
            }
        } else {
            Some(format!("{} annotations selected", ids.len()))
        }
    };

    view! {
        <div class="analysis-panel">
            {move || {
                let has_file = state.current_file_index.get().is_some() || state.active_timeline.get().is_some();

                if !has_file {
                    return view! {
                        <span style="color: #555">"Load a file..."</span>
                    }.into_any();
                }

                // Selection dimensions take priority
                if let Some(dims) = selection_dims() {
                    return view! {
                        <span>{dims}</span>
                    }.into_any();
                }

                // Selected annotation dimensions
                if let Some(dims) = annotation_dims() {
                    return view! {
                        <span style="color: #aaa">{dims}</span>
                    }.into_any();
                }

                // FF handle interaction
                if state.spec_drag_handle.get().is_some() {
                    return view! {
                        <span style="color: #888">"Adjusting frequency focus"</span>
                    }.into_any();
                }

                // Axis drag
                if state.axis_drag_start_freq.get().is_some() {
                    return view! {
                        <span style="color: #888">"Selecting frequency range..."</span>
                    }.into_any();
                }

                // Drag in progress
                if state.is_dragging.get() {
                    let msg = match state.canvas_tool.get() {
                        CanvasTool::Hand => "Panning...",
                        CanvasTool::Selection => "Selecting...",
                    };
                    return view! {
                        <span style="color: #888">{msg}</span>
                    }.into_any();
                }

                // Hovering label area
                if state.mouse_in_label_area.get() {
                    return view! {
                        <span style="color: #666">"Drag to set frequency focus"</span>
                    }.into_any();
                }

                // Mouse on spectrogram: show time and frequency
                let freq = state.mouse_freq.get();
                let time = state.cursor_time.get();
                if let (Some(f), Some(t)) = (freq, time) {
                    let freq_str = if f >= 1000.0 {
                        format!("{:.1} kHz", f / 1000.0)
                    } else {
                        format!("{:.0} Hz", f)
                    };
                    return view! {
                        <span style="color: #777">{format!("{:.3}s  {}", t, freq_str)}</span>
                    }.into_any();
                }

                // Default: empty
                view! {
                    <span></span>
                }.into_any()
            }}
        </div>
    }
}
