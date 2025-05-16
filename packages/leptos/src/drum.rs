use leptos::{prelude::*, svg::Svg};
#[component]
pub fn Drum(
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
            <path d="m2 2 8 8"></path>
            <path d="m22 2-8 8"></path>
            <ellipse cx="12" cy="9" rx="10" ry="5"></ellipse>
            <path d="M7 13.4v7.9"></path>
            <path d="M12 14v8"></path>
            <path d="M17 13.4v7.9"></path>
            <path d="M2 9v8a10 5 0 0 0 20 0V9"></path>
        </svg>
    }
}
