use leptos::{prelude::*, svg::Svg};
#[component]
pub fn Heater(
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
            <path d="M11 8c2-3-2-3 0-6"></path>
            <path d="M15.5 8c2-3-2-3 0-6"></path>
            <path d="M6 10h.01"></path>
            <path d="M6 14h.01"></path>
            <path d="M10 16v-4"></path>
            <path d="M14 16v-4"></path>
            <path d="M18 16v-4"></path>
            <path d="M20 6a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h3"></path>
            <path d="M5 20v2"></path>
            <path d="M19 20v2"></path>
        </svg>
    }
}
