use leptos::{prelude::*, svg::Svg};
#[component]
pub fn GitBranchPlus(
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
            <path d="M6 3v12" />
            <path d="M18 9a3 3 0 1 0 0-6 3 3 0 0 0 0 6z" />
            <path d="M6 21a3 3 0 1 0 0-6 3 3 0 0 0 0 6z" />
            <path d="M15 6a9 9 0 0 0-9 9" />
            <path d="M18 15v6" />
            <path d="M21 18h-6" />
        </svg>
    }
}
