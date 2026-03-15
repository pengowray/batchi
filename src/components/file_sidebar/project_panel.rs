use leptos::prelude::*;

#[component]
pub fn ProjectPanel() -> impl IntoView {
    view! {
        <div class="project-panel">
            <div class="project-panel-empty">
                <p>"No project open"</p>
                <p class="project-panel-hint">"Load audio files and create a project to group them together."</p>
            </div>
        </div>
    }
}
