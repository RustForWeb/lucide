use leptos::{prelude::*, svg::Svg};
#[component]
pub fn ScanQrCode(
    #[prop(default = 24.into(), into)] size: Signal<usize>,
    #[prop(default = "currentColor".into(), into)] color: Signal<String>,
    #[prop(default = "none".into(), into)] fill: Signal<String>,
    #[prop(default = 2.into(), into)] stroke_width: Signal<usize>,
    #[prop(default = false.into(), into)] absolute_stroke_width: Signal<bool>,
    #[prop(optional)] node_ref: NodeRef<Svg>,
) -> impl IntoView {
    let stroke_width = Signal::derive(move || {
        if absolute_stroke_width.get() {
            stroke_width.get() * 24 / size.get()
        } else {
            stroke_width.get()
        }
    });
    view! {
        <svg
            node_ref=node_ref
            class:lucide=true
            xmlns="http://www.w3.org/2000/svg"
            width=size
            height=size
            viewBox="0 0 24 24"
            fill=fill
            stroke=color
            stroke-width=stroke_width
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="M17 12v4a1 1 0 0 1-1 1h-4"></path>
            <path d="M17 3h2a2 2 0 0 1 2 2v2"></path>
            <path d="M17 8V7"></path>
            <path d="M21 17v2a2 2 0 0 1-2 2h-2"></path>
            <path d="M3 7V5a2 2 0 0 1 2-2h2"></path>
            <path d="M7 17h.01"></path>
            <path d="M7 21H5a2 2 0 0 1-2-2v-2"></path>
            <rect x="7" y="7" width="5" height="5" rx="1"></rect>
        </svg>
    }
}
