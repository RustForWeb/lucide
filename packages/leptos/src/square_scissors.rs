use leptos::{prelude::*, svg::Svg};
#[component]
pub fn SquareScissors(
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
            <path d="m17 17-2.18-2.18" />
            <path d="M9.56 14.44 17 7" />
            <path d="M9.56 9.56 12 12" />
            <circle cx="8.5" cy="15.5" r="1.5" />
            <circle cx="8.5" cy="8.5" r="1.5" />
            <rect x="3" y="3" width="18" height="18" rx="2" />
        </svg>
    }
}
