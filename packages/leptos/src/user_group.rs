use leptos::{prelude::*, svg::Svg};
#[component]
pub fn UserGroup(
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
            <path d="M17 21v-1a2 2 0 00-2-2H9a2 2 0 00-2 2v1" />
            <path d="M19 10h1a2 2 0 012 2v1" />
            <path d="M5 10H4a2 2 0 00-2 2v1" />
            <circle cx="12" cy="11" r="3" />
            <circle cx="18" cy="4" r="2" />
            <circle cx="6" cy="4" r="2" />
        </svg>
    }
}
