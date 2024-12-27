use leptos::{prelude::*, svg::Svg};
#[component]
pub fn ShowerHead(
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
            <path d="m4 4 2.5 2.5" />
            <path d="M13.5 6.5a4.95 4.95 0 0 0-7 7" />
            <path d="M15 5 5 15" />
            <path d="M14 17v.01" />
            <path d="M10 16v.01" />
            <path d="M13 13v.01" />
            <path d="M16 10v.01" />
            <path d="M11 20v.01" />
            <path d="M17 14v.01" />
            <path d="M20 11v.01" />
        </svg>
    }
}
