use leptos::{prelude::*, svg::Svg};
#[component]
pub fn ChartNetwork(
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
            <path d="m13.11 7.664 1.78 2.672"></path>
            <path d="m14.162 12.788-3.324 1.424"></path>
            <path d="m20 4-6.06 1.515"></path>
            <path d="M3 3v16a2 2 0 0 0 2 2h16"></path>
            <circle cx="12" cy="6" r="2"></circle>
            <circle cx="16" cy="12" r="2"></circle>
            <circle cx="9" cy="15" r="2"></circle>
        </svg>
    }
}
