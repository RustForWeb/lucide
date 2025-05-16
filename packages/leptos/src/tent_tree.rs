use leptos::{prelude::*, svg::Svg};
#[component]
pub fn TentTree(
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
            <circle cx="4" cy="4" r="2"></circle>
            <path d="m14 5 3-3 3 3"></path>
            <path d="m14 10 3-3 3 3"></path>
            <path d="M17 14V2"></path>
            <path d="M17 14H7l-5 8h20Z"></path>
            <path d="M8 14v8"></path>
            <path d="m9 14 5 8"></path>
        </svg>
    }
}
