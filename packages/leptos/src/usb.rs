use leptos::{prelude::*, svg::Svg};
#[component]
pub fn Usb(
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
            <circle cx="10" cy="7" r="1" />
            <circle cx="4" cy="20" r="1" />
            <path d="M4.7 19.3 19 5" />
            <path d="m21 3-3 1 2 2Z" />
            <path d="M9.26 7.68 5 12l2 5" />
            <path d="m10 14 5 2 3.5-3.5" />
            <path d="m18 12 1-1 1 1-1 1Z" />
        </svg>
    }
}
