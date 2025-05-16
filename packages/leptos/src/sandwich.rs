use leptos::{prelude::*, svg::Svg};
#[component]
pub fn Sandwich(
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
            <path d="m2.37 11.223 8.372-6.777a2 2 0 0 1 2.516 0l8.371 6.777"></path>
            <path d="M21 15a1 1 0 0 1 1 1v2a1 1 0 0 1-1 1h-5.25"></path>
            <path d="M3 15a1 1 0 0 0-1 1v2a1 1 0 0 0 1 1h9"></path>
            <path d="m6.67 15 6.13 4.6a2 2 0 0 0 2.8-.4l3.15-4.2"></path>
            <rect width="20" height="4" x="2" y="11" rx="1"></rect>
        </svg>
    }
}
