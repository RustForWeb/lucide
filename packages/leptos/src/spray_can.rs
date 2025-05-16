use leptos::{prelude::*, svg::Svg};
#[component]
pub fn SprayCan(
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
            <path d="M3 3h.01"></path>
            <path d="M7 5h.01"></path>
            <path d="M11 7h.01"></path>
            <path d="M3 7h.01"></path>
            <path d="M7 9h.01"></path>
            <path d="M3 11h.01"></path>
            <rect width="4" height="4" x="15" y="5"></rect>
            <path d="m19 9 2 2v10c0 .6-.4 1-1 1h-6c-.6 0-1-.4-1-1V11l2-2"></path>
            <path d="m13 14 8-2"></path>
            <path d="m13 19 8-2"></path>
        </svg>
    }
}
