use leptos::{prelude::*, svg::Svg};
#[component]
pub fn Snowflake(
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
            <line x1="2" x2="22" y1="12" y2="12"></line>
            <line x1="12" x2="12" y1="2" y2="22"></line>
            <path d="m20 16-4-4 4-4"></path>
            <path d="m4 8 4 4-4 4"></path>
            <path d="m16 4-4 4-4-4"></path>
            <path d="m8 20 4-4 4 4"></path>
        </svg>
    }
}