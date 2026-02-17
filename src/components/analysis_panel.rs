use leptos::prelude::*;
use crate::state::AppState;

#[component]
pub fn AnalysisPanel() -> impl IntoView {
    let state = expect_context::<AppState>();

    let duration = move || {
        let selection = state.selection.get()?;
        let d = selection.time_end - selection.time_start;
        if d > 0.0001 { Some(d) } else { None }
    };

    view! {
        <div class="analysis-panel">
            {move || {
                match duration() {
                    Some(d) => {
                        view! {
                            <span>{format!("{:.3}s", d)}</span>
                        }.into_any()
                    }
                    None => {
                        view! {
                            <span style="color: #555">"No selection"</span>
                        }.into_any()
                    }
                }
            }}
        </div>
    }
}
