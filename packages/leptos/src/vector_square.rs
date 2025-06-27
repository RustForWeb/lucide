use leptos::{prelude::*, svg::Svg};
#[component]
pub fn VectorSquare(
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
            <path d="M19.5 7a24 24 0 0 1 0 10" />
            <path d="M4.5 7a24 24 0 0 0 0 10" />
            <path d="M7 19.5a24 24 0 0 0 10 0" />
            <path d="M7 4.5a24 24 0 0 1 10 0" />
            <rect x="17" y="17" width="5" height="5" rx="1" />
            <rect x="17" y="2" width="5" height="5" rx="1" />
            <rect x="2" y="17" width="5" height="5" rx="1" />
            <rect x="2" y="2" width="5" height="5" rx="1" />
        </svg>
    }
}
