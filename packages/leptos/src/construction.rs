use leptos::{prelude::*, svg::Svg};
#[component]
pub fn Construction(
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
            <rect x="2" y="6" width="20" height="8" rx="1"></rect>
            <path d="M17 14v7"></path>
            <path d="M7 14v7"></path>
            <path d="M17 3v3"></path>
            <path d="M7 3v3"></path>
            <path d="M10 14 2.3 6.3"></path>
            <path d="m14 6 7.7 7.7"></path>
            <path d="m8 6 8 8"></path>
        </svg>
    }
}
