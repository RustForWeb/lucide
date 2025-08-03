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
            <path d="M10.1 2.182a10 10 0 0 1 3.8 0" />
            <path d="M13.9 21.818a10 10 0 0 1-3.8 0" />
            <path d="M17.609 3.72a10 10 0 0 1 2.69 2.7" />
            <path d="M2.182 13.9a10 10 0 0 1 0-3.8" />
            <path d="M20.28 17.61a10 10 0 0 1-2.7 2.69" />
            <path d="M21.818 10.1a10 10 0 0 1 0 3.8" />
            <path d="M3.721 6.391a10 10 0 0 1 2.7-2.69" />
            <path d="m6.163 21.117-2.906.85a1 1 0 0 1-1.236-1.169l.965-2.98" />
        </svg>
    }
}
