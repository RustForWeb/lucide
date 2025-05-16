use leptos::{prelude::*, svg::Svg};
#[component]
pub fn Shredder(
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
            <path d="M10 22v-5" />
            <path d="M14 19v-2" />
            <path d="M14 2v4a2 2 0 0 0 2 2h4" />
            <path d="M18 20v-3" />
            <path d="M2 13h20" />
            <path d="M20 13V7l-5-5H6a2 2 0 0 0-2 2v9" />
            <path d="M6 20v-3" />
        </svg>
    }
}
