use leptos::{prelude::*, svg::Svg};
#[component]
pub fn TextSelect(
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
            <path d="M14 21h1"></path>
            <path d="M14 3h1"></path>
            <path d="M19 3a2 2 0 0 1 2 2"></path>
            <path d="M21 14v1"></path>
            <path d="M21 19a2 2 0 0 1-2 2"></path>
            <path d="M21 9v1"></path>
            <path d="M3 14v1"></path>
            <path d="M3 9v1"></path>
            <path d="M5 21a2 2 0 0 1-2-2"></path>
            <path d="M5 3a2 2 0 0 0-2 2"></path>
            <path d="M7 12h10"></path>
            <path d="M7 16h6"></path>
            <path d="M7 8h8"></path>
            <path d="M9 21h1"></path>
            <path d="M9 3h1"></path>
        </svg>
    }
}
