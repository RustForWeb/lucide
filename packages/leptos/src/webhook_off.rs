use leptos::{prelude::*, svg::Svg};
#[component]
pub fn WebhookOff(
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
            <path d="M17 17h-5c-1.09-.02-1.94.92-2.5 1.9A3 3 0 1 1 2.57 15"></path>
            <path d="M9 3.4a4 4 0 0 1 6.52.66"></path>
            <path d="m6 17 3.1-5.8a2.5 2.5 0 0 0 .057-2.05"></path>
            <path d="M20.3 20.3a4 4 0 0 1-2.3.7"></path>
            <path d="M18.6 13a4 4 0 0 1 3.357 3.414"></path>
            <path d="m12 6 .6 1"></path>
            <path d="m2 2 20 20"></path>
        </svg>
    }
}
