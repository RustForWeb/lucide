use leptos::{prelude::*, svg::Svg};
#[component]
pub fn CircleDotDashed(
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
            <path d="M10.1 2.18a9.93 9.93 0 0 1 3.8 0"></path>
            <path d="M17.6 3.71a9.95 9.95 0 0 1 2.69 2.7"></path>
            <path d="M21.82 10.1a9.93 9.93 0 0 1 0 3.8"></path>
            <path d="M20.29 17.6a9.95 9.95 0 0 1-2.7 2.69"></path>
            <path d="M13.9 21.82a9.94 9.94 0 0 1-3.8 0"></path>
            <path d="M6.4 20.29a9.95 9.95 0 0 1-2.69-2.7"></path>
            <path d="M2.18 13.9a9.93 9.93 0 0 1 0-3.8"></path>
            <path d="M3.71 6.4a9.95 9.95 0 0 1 2.7-2.69"></path>
            <circle cx="12" cy="12" r="1"></circle>
        </svg>
    }
}
