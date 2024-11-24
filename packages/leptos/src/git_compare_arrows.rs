use leptos::{prelude::*, svg::Svg};
#[component]
pub fn GitCompareArrows(
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
            <circle cx="5" cy="6" r="3"></circle>
            <path d="M12 6h5a2 2 0 0 1 2 2v7"></path>
            <path d="m15 9-3-3 3-3"></path>
            <circle cx="19" cy="18" r="3"></circle>
            <path d="M12 18H7a2 2 0 0 1-2-2V9"></path>
            <path d="m9 15 3 3-3 3"></path>
        </svg>
    }
}