use leptos::{prelude::*, svg::Svg};
#[component]
pub fn ImageUpscale(
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
            <path d="M16 3h5v5"></path>
            <path d="M17 21h2a2 2 0 0 0 2-2"></path>
            <path d="M21 12v3"></path>
            <path d="m21 3-5 5"></path>
            <path d="M3 7V5a2 2 0 0 1 2-2"></path>
            <path d="m5 21 4.144-4.144a1.21 1.21 0 0 1 1.712 0L13 19"></path>
            <path d="M9 3h3"></path>
            <rect x="3" y="11" width="10" height="10" rx="1"></rect>
        </svg>
    }
}
