use leptos::{prelude::*, svg::Svg};
#[component]
pub fn Dna(
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
            <path d="m10 16 1.5 1.5" />
            <path d="m14 8-1.5-1.5" />
            <path d="M15 2c-1.798 1.998-2.518 3.995-2.807 5.993" />
            <path d="m16.5 10.5 1 1" />
            <path d="m17 6-2.891-2.891" />
            <path d="M2 15c6.667-6 13.333 0 20-6" />
            <path d="m20 9 .891.891" />
            <path d="M3.109 14.109 4 15" />
            <path d="m6.5 12.5 1 1" />
            <path d="m7 18 2.891 2.891" />
            <path d="M9 22c1.798-1.998 2.518-3.995 2.807-5.993" />
        </svg>
    }
}
