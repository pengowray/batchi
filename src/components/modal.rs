// Modal CSS class convention (no component — Leptos 0.8 AnyView isn't Clone).
//
// Use the about-dialog pattern: conditional `{move || signal.get().then(|| view! { ... })}`.
//
// CSS classes for consistent modals:
//   .modal-overlay   — fixed fullscreen backdrop, click-to-dismiss
//   .modal-dialog    — centered container with dark bg, rounded corners
//   .modal-header    — flex row: title + close button
//   .modal-title     — header text
//   .modal-close     — × button
//   .modal-body      — scrollable content area
//
// Example:
//   {move || show.get().then(|| view! {
//       <div class="modal-overlay" on:click=move |_| show.set(false)>
//           <div class="modal-dialog" on:click=move |ev: web_sys::MouseEvent| ev.stop_propagation()>
//               <div class="modal-header">
//                   <span class="modal-title">"Title"</span>
//                   <button class="modal-close" on:click=move |_| show.set(false)>{"\u{00D7}"}</button>
//               </div>
//               <div class="modal-body">
//                   <p>"Content here"</p>
//               </div>
//           </div>
//       </div>
//   })}
