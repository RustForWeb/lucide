use leptos::{prelude::*, svg::Svg};
#[component]
pub fn Slack(
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
            <rect width="3" height="8" x="13" y="2" rx="1.5" />
            <path d="M19 8.5V10h1.5A1.5 1.5 0 1 0 19 8.5" />
            <rect width="3" height="8" x="8" y="14" rx="1.5" />
            <path d="M5 15.5V14H3.5A1.5 1.5 0 1 0 5 15.5" />
            <rect width="8" height="3" x="14" y="13" rx="1.5" />
            <path d="M15.5 19H14v1.5a1.5 1.5 0 1 0 1.5-1.5" />
            <rect width="8" height="3" x="2" y="8" rx="1.5" />
            <path d="M8.5 5H10V3.5A1.5 1.5 0 1 0 8.5 5" />
        </svg>
    }
}
