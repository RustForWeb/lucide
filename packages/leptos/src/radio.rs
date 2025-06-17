use leptos::{prelude::*, svg::Svg};
#[component]
pub fn Radio(
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
            <path d="M16.247 7.761a6 6 0 0 1 0 8.478" />
            <path d="M19.075 4.933a10 10 0 0 1 0 14.134" />
            <path d="M4.925 19.067a10 10 0 0 1 0-14.134" />
            <path d="M7.753 16.239a6 6 0 0 1 0-8.478" />
            <circle cx="12" cy="12" r="2" />
        </svg>
    }
}
