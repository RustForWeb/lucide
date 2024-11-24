use leptos::{prelude::*, svg::Svg};
#[component]
pub fn DnaOff(
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
            <path d="M15 2c-1.35 1.5-2.092 3-2.5 4.5L14 8"></path>
            <path d="m17 6-2.891-2.891"></path>
            <path d="M2 15c3.333-3 6.667-3 10-3"></path>
            <path d="m2 2 20 20"></path>
            <path d="m20 9 .891.891"></path>
            <path d="M22 9c-1.5 1.35-3 2.092-4.5 2.5l-1-1"></path>
            <path d="M3.109 14.109 4 15"></path>
            <path d="m6.5 12.5 1 1"></path>
            <path d="m7 18 2.891 2.891"></path>
            <path d="M9 22c1.35-1.5 2.092-3 2.5-4.5L10 16"></path>
        </svg>
    }
}
