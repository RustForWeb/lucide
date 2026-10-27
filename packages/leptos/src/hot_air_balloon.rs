use leptos::{prelude::*, svg::Svg};
#[component]
pub fn HotAirBalloon(
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
            <path d="M11.5 18C10 15 9 12 9 9a3 7 0 0 1 6 0c0 3-1 6-2.5 9" />
            <path d="M15 18c3-3 5-5 5-9A8 7 0 0 0 4 9c0 4 2 6 5 9" />
            <path d="M15 18H9v3a1 1 0 0 0 1 1h4a1 1 0 0 0 1-1z" />
        </svg>
    }
}
