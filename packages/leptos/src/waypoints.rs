use leptos::{prelude::*, svg::Svg};
#[component]
pub fn Waypoints(
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
            <circle cx="12" cy="4.5" r="2.5" />
            <path d="m10.2 6.3-3.9 3.9" />
            <circle cx="4.5" cy="12" r="2.5" />
            <path d="M7 12h10" />
            <circle cx="19.5" cy="12" r="2.5" />
            <path d="m13.8 17.7 3.9-3.9" />
            <circle cx="12" cy="19.5" r="2.5" />
        </svg>
    }
}
