use leptos::{prelude::*, svg::Svg};
#[component]
pub fn TicketsPlane(
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
            <path d="M10.5 17h1.227a2 2 0 0 0 1.345-.52L18 12"></path>
            <path d="m12 13.5 3.75.5"></path>
            <path d="m4.5 8 10.58-5.06a1 1 0 0 1 1.342.488L18.5 8"></path>
            <path d="M6 10V8"></path>
            <path d="M6 14v1"></path>
            <path d="M6 19v2"></path>
            <rect x="2" y="8" width="20" height="13" rx="2"></rect>
        </svg>
    }
}
