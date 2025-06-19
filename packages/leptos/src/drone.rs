use leptos::{prelude::*, svg::Svg};
#[component]
pub fn Drone(
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
            <path d="M10 10 7 7" />
            <path d="m10 14-3 3" />
            <path d="m14 10 3-3" />
            <path d="m14 14 3 3" />
            <path d="M14.205 4.139a4 4 0 1 1 5.439 5.863" />
            <path d="M19.637 14a4 4 0 1 1-5.432 5.868" />
            <path d="M4.367 10a4 4 0 1 1 5.438-5.862" />
            <path d="M9.795 19.862a4 4 0 1 1-5.429-5.873" />
            <rect x="10" y="8" width="4" height="8" rx="1" />
        </svg>
    }
}
