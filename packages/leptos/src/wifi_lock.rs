use leptos::{prelude::*, svg::Svg};
#[component]
pub fn WifiLock(
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
            <path d="M2 8.82a15 15 0 0120 0" />
            <path d="M20 16v-2a2 2 0 00-4 0v2" />
            <path d="M5 12.859a10 10 0 018.436-2.756" />
            <path d="M8.5 16.429a5 5 0 011.794-1.13" />
            <rect x="14" y="16" width="8" height="5" rx="1" />
        </svg>
    }
}
