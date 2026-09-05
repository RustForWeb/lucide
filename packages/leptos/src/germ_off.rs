use leptos::{prelude::*, svg::Svg};
#[component]
pub fn GermOff(
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
            <path d="m11 2 .925 1.848" />
            <path d="M13 15h.01" />
            <path d="M13.424 7.768a2 2 0 112.808 2.808" />
            <path d="m16 21-1-2.472" />
            <path d="M16.988 16.988A12 12 0 019 20a5 5 0 01-2.759-9.171 8.8 8.8 0 002.307-2.28" />
            <path d="m19 2-1 1.804" />
            <path d="m2 19 2.746-1.373" />
            <path d="m2 2 20 20" />
            <path d="m22 16-2.474-2.13a12 12 0 001.376-3.786 6 6 0 00-10.313-5.151" />
            <path d="m22 5-1.804 1" />
            <path d="m3 10 2 2" />
            <path d="M9 16h.01" />
            <path d="M9 20v2" />
        </svg>
    }
}
