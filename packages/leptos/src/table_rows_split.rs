use leptos::{prelude::*, svg::Svg};
#[component]
pub fn TableRowsSplit(
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
            <path d="M14 10h2"></path>
            <path d="M15 22v-8"></path>
            <path d="M15 2v4"></path>
            <path d="M2 10h2"></path>
            <path d="M20 10h2"></path>
            <path d="M3 19h18"></path>
            <path d="M3 22v-6a2 2 135 0 1 2-2h14a2 2 45 0 1 2 2v6"></path>
            <path d="M3 2v2a2 2 45 0 0 2 2h14a2 2 135 0 0 2-2V2"></path>
            <path d="M8 10h2"></path>
            <path d="M9 22v-8"></path>
            <path d="M9 2v4"></path>
        </svg>
    }
}
