use leptos::{prelude::*, svg::Svg};
#[component]
pub fn ShrimpOff(
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
            <path d="M10 2a3.28 3.28 0 003.227 1.798l6.17-.561A1 1 0 1119.614 8H13.5" />
            <path d="M11 20c-.5.5-1.12 1-2.5 1a1 1 0 010-5H12a7 7 0 003.283-.817" />
            <path d="M11 22c-.5-.5-1.12-1-2.5-1a6.5 6.5 0 01-5.63-3.25 6.44 6.44 0 015.236-9.744" />
            <path d="M18.04 12.54A7 7 0 0019 9V8" />
            <path d="m2 2 20 20" />
            <path d="M8 16c-2 0-4.5-4-4-6" />
            <path d="M9.43 9.33A8.5 8.5 0 0010 16" />
        </svg>
    }
}
