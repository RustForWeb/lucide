use leptos::{prelude::*, svg::Svg};
#[component]
pub fn Shrink(
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
            <path d="m15 15 6 6m-6-6v4.8m0-4.8h4.8"></path>
            <path d="M9 19.8V15m0 0H4.2M9 15l-6 6"></path>
            <path d="M15 4.2V9m0 0h4.8M15 9l6-6"></path>
            <path d="M9 4.2V9m0 0H4.2M9 9 3 3"></path>
        </svg>
    }
}
