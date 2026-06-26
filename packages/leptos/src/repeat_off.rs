use leptos::{prelude::*, svg::Svg};
#[component]
pub fn RepeatOff(
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
            <path d="M11.656 6H21l-4-4" />
            <path d="M17.898 17.898A4 4 0 0 1 17 18H3l4-4" />
            <path d="m2 2 20 20" />
            <path d="M21 13v1a4 4 0 0 1-.171 1.159" />
            <path d="m21 6-4 4" />
            <path d="M3 11v-1a4 4 0 0 1 3.102-3.898" />
            <path d="m7 22-4-4" />
        </svg>
    }
}
