use leptos::{prelude::*, svg::Svg};
#[component]
pub fn CalendarCheck2(
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
            <path d="M 19 3 L 5 3" />
            <path d="M 21 13 L 21 5" />
            <path d="M 21 5 A2 2 0 0 0 19 3" />
            <path d="M 3 19 A2 2 0 0 0 5 21" />
            <path d="M 3 5 L 3 19" />
            <path d="M 5 3 A2 2 0 0 0 3 5" />
            <path d="m16 19 2 2 4-4" />
            <path d="M16 2v3" />
            <path d="M3 9h18" />
            <path d="M5 21 L12.5 21" />
            <path d="M8 2v3" />
        </svg>
    }
}
