use leptos::{prelude::*, svg::Svg};
#[component]
pub fn ShipWheel(
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
            <circle cx="12" cy="12" r="8"></circle>
            <path d="M12 2v7.5"></path>
            <path d="m19 5-5.23 5.23"></path>
            <path d="M22 12h-7.5"></path>
            <path d="m19 19-5.23-5.23"></path>
            <path d="M12 14.5V22"></path>
            <path d="M10.23 13.77 5 19"></path>
            <path d="M9.5 12H2"></path>
            <path d="M10.23 10.23 5 5"></path>
            <circle cx="12" cy="12" r="2.5"></circle>
        </svg>
    }
}