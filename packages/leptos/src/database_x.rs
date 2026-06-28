use leptos::{prelude::*, svg::Svg};
#[component]
pub fn DatabaseX(
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
            <path d="m17 17 5 5" />
            <path d="M19.323 13.744A9 3 0 0 0 21 12" />
            <path d="M21 13.127V5" />
            <path d="m22 17-5 5" />
            <path d="M3 12A9 3 0 0 0 13.563 14.954" />
            <path d="M3 5V19A9 3 0 0 0 13 21.981" />
            <ellipse cx="12" cy="5" rx="9" ry="3" />
        </svg>
    }
}
