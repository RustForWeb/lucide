use leptos::{prelude::*, svg::Svg};
#[component]
pub fn ChartQuadrant(
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
            <path d="M13 3v18" />
            <path d="M3 12h18" />
            <path d="M3 3v16a2 2 0 002 2h16" />
            <circle cx="17.5" cy="5.5" r="0.5" fill="currentColor" />
            <circle cx="19.5" cy="16.5" r="0.5" fill="currentColor" />
            <circle cx="7.5" cy="16.5" r="0.5" fill="currentColor" />
            <circle cx="7.5" cy="7.5" r="0.5" fill="currentColor" />
        </svg>
    }
}
