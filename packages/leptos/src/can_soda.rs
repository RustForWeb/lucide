use leptos::{prelude::*, svg::Svg};
#[component]
pub fn CanSoda(
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
            <path d="m17 22 1.664-2.496a2 2 0 00.336-1.11V5.606a2 2 0 00-.336-1.11L17 2" />
            <path d="M18 22H6" />
            <path d="M18 2H6" />
            <path d="M5 17h14" />
            <path d="M5 7h14" />
            <path d="m7 22-1.664-2.496A2 2 0 015 18.394V5.606a2 2 0 01.336-1.11L7 2" />
        </svg>
    }
}
