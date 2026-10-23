use leptos::{prelude::*, svg::Svg};
#[component]
pub fn BrainOff(
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
            <path d="M12 18v-6" />
            <path d="M12 5v1.344" />
            <path d="M17.707 6.293A3 3 0 1012 5a3 3 0 00-4.124-2.781" />
            <path d="M17.997 5.125a4 4 0 012.526 5.77" />
            <path d="M19.644 19.644A4 4 0 0112 18a4 4 0 11-7.967-.517" />
            <path d="m2 2 20 20" />
            <path d="M20 10.536a4 4 0 011.533 5.34" />
            <path d="M3.477 10.895a4 4 0 011.876-5.542" />
            <path d="M6 18a4 4 0 01-2-7.464" />
            <path d="M9 13a4.17 4.17 0 002.166-1.5 4 4 0 00.138-.196" />
        </svg>
    }
}
