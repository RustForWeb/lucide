use leptos::{prelude::*, svg::Svg};
#[component]
pub fn Orbit(
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
            <path d="M20.341 6.484A10 10 0 0 1 10.266 21.85"></path>
            <path d="M3.659 17.516A10 10 0 0 1 13.74 2.152"></path>
            <circle cx="12" cy="12" r="3"></circle>
            <circle cx="19" cy="5" r="2"></circle>
            <circle cx="5" cy="19" r="2"></circle>
        </svg>
    }
}
