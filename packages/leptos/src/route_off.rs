use leptos::{prelude::*, svg::Svg};
#[component]
pub fn RouteOff(
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
            <circle cx="6" cy="19" r="3" />
            <path d="M9 19h8.5c.4 0 .9-.1 1.3-.2" />
            <path d="M5.2 5.2A3.5 3.53 0 0 0 6.5 12H12" />
            <path d="m2 2 20 20" />
            <path d="M21 15.3a3.5 3.5 0 0 0-3.3-3.3" />
            <path d="M15 5h-4.3" />
            <circle cx="18" cy="5" r="3" />
        </svg>
    }
}
