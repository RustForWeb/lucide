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
            <rect width="18" height="18" x="3" y="3" rx="2" />
            <circle cx="8.5" cy="8.5" r="1.5" />
            <line x1="9.56066" y1="9.56066" x2="12" y2="12" />
            <line x1="17" y1="17" x2="14.82" y2="14.82" />
            <circle cx="8.5" cy="15.5" r="1.5" />
            <line x1="9.56066" y1="14.43934" x2="17" y2="7" />
        </svg>
    }
}
