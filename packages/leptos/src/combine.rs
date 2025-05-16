use leptos::{prelude::*, svg::Svg};
#[component]
pub fn Combine(
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
            <path d="M10 18H5a3 3 0 0 1-3-3v-1"></path>
            <path d="M14 2a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2"></path>
            <path d="M20 2a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2"></path>
            <path d="m7 21 3-3-3-3"></path>
            <rect x="14" y="14" width="8" height="8" rx="2"></rect>
            <rect x="2" y="2" width="8" height="8" rx="2"></rect>
        </svg>
    }
}
