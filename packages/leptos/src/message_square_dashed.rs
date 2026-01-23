use leptos::{prelude::*, svg::Svg};
#[component]
pub fn MessageSquareDashed(
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
            <path d="M14 3h2" />
            <path d="M16 19h-2" />
            <path d="M2 12v-2" />
            <path d="M2 16v5.286a.71.71 0 0 0 1.212.502l1.149-1.149" />
            <path d="M20 19a2 2 0 0 0 2-2v-1" />
            <path d="M22 10v2" />
            <path d="M22 6V5a2 2 0 0 0-2-2" />
            <path d="M4 3a2 2 0 0 0-2 2v1" />
            <path d="M8 19h2" />
            <path d="M8 3h2" />
        </svg>
    }
}
