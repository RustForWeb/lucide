use leptos::{prelude::*, svg::Svg};
#[component]
pub fn SquaresIntersect(
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
            <path d="M10 22a2 2 0 0 1-2-2" />
            <path d="M14 2a2 2 0 0 1 2 2" />
            <path d="M16 22h-2" />
            <path d="M2 10V8" />
            <path d="M2 4a2 2 0 0 1 2-2" />
            <path d="M20 8a2 2 0 0 1 2 2" />
            <path d="M22 14v2" />
            <path d="M22 20a2 2 0 0 1-2 2" />
            <path d="M4 16a2 2 0 0 1-2-2" />
            <path d="M8 10a2 2 0 0 1 2-2h5a1 1 0 0 1 1 1v5a2 2 0 0 1-2 2H9a1 1 0 0 1-1-1z" />
            <path d="M8 2h2" />
        </svg>
    }
}
