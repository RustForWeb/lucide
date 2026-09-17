use leptos::{prelude::*, svg::Svg};
#[component]
pub fn BuildingComplexPlus(
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
            <path d="M10 12h4" />
            <path d="M10 21v-3a2 2 0 013.05-1.702" />
            <path d="M10 8h4" />
            <path d="M16 19h6" />
            <path d="M18 7h2a2 2 0 012 2v4.355" />
            <path d="M19 16v6" />
            <path d="M6 10H4a2 2 0 00-2 2v7a2 2 0 002 2h8.535" />
            <path d="M6 21V5a2 2 0 012-2h8a2 2 0 012 2v7.126" />
        </svg>
    }
}
