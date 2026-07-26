use leptos::{prelude::*, svg::Svg};
#[component]
pub fn Barrel(
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
            <path d="M10 3a41 41 0 000 18" />
            <path d="M14 3a41 41 0 010 18" />
            <path d="M16.997 21a2 2 0 001.68-.92 15.25 15.25 0 000-16.16 2 2 0 00-1.68-.92h-10a2 2 0 00-1.681.92 15.25 15.25 0 000 16.16 2 2 0 001.681.92z" />
            <path d="M3.54 16h16.914" />
            <path d="M3.54 8h16.914" />
        </svg>
    }
}
