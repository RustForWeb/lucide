use leptos::{prelude::*, svg::Svg};
#[component]
pub fn MessageCircleDashed(
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
            <path d="M13.5 3.1c-.5 0-1-.1-1.5-.1s-1 .1-1.5.1" />
            <path d="M19.3 6.8a10.45 10.45 0 0 0-2.1-2.1" />
            <path d="M20.9 13.5c.1-.5.1-1 .1-1.5s-.1-1-.1-1.5" />
            <path d="M17.2 19.3a10.45 10.45 0 0 0 2.1-2.1" />
            <path d="M10.5 20.9c.5.1 1 .1 1.5.1s1-.1 1.5-.1" />
            <path d="M3.5 17.5 2 22l4.5-1.5" />
            <path d="M3.1 10.5c0 .5-.1 1-.1 1.5s.1 1 .1 1.5" />
            <path d="M6.8 4.7a10.45 10.45 0 0 0-2.1 2.1" />
        </svg>
    }
}
