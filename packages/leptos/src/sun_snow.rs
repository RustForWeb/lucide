use leptos::{prelude::*, svg::Svg};
#[component]
pub fn SunSnow(
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
            <path d="M10 9a3 3 0 1 0 0 6"></path>
            <path d="M2 12h1"></path>
            <path d="M14 21V3"></path>
            <path d="M10 4V3"></path>
            <path d="M10 21v-1"></path>
            <path d="m3.64 18.36.7-.7"></path>
            <path d="m4.34 6.34-.7-.7"></path>
            <path d="M14 12h8"></path>
            <path d="m17 4-3 3"></path>
            <path d="m14 17 3 3"></path>
            <path d="m21 15-3-3 3-3"></path>
        </svg>
    }
}