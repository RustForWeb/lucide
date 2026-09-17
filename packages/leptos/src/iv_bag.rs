use leptos::{prelude::*, svg::Svg};
#[component]
pub fn IvBag(
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
            <path d="M12 18v2a2 2 0 002 2h6" />
            <path d="M6 11c.72.5 1.44 1 3 1 3 0 3-2 6-2 1.56 0 2.28.5 3 1" />
            <path d="M9.293 3c.453 0 .887-.18 1.207-.5s.754-.5 1.207-.5h.586c.453 0 .887.18 1.207.5s.754.5 1.207.5H16a2 2 0 012 2v11a2 2 0 01-2 2H8a2 2 0 01-2-2V5a2 2 0 012-2z" />
        </svg>
    }
}
