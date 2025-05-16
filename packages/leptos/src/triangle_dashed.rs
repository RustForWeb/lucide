use leptos::{prelude::*, svg::Svg};
#[component]
pub fn TriangleDashed(
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
            <path d="M10.17 4.193a2 2 0 0 1 3.666.013" />
            <path d="M14 21h2" />
            <path d="m15.874 7.743 1 1.732" />
            <path d="m18.849 12.952 1 1.732" />
            <path d="M21.824 18.18a2 2 0 0 1-1.835 2.824" />
            <path d="M4.024 21a2 2 0 0 1-1.839-2.839" />
            <path d="m5.136 12.952-1 1.732" />
            <path d="M8 21h2" />
            <path d="m8.102 7.743-1 1.732" />
        </svg>
    }
}
