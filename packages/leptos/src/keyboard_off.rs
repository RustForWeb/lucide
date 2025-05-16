use leptos::{prelude::*, svg::Svg};
#[component]
pub fn KeyboardOff(
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
            <path d="M 20 4 A2 2 0 0 1 22 6"></path>
            <path d="M 22 6 L 22 16.41"></path>
            <path d="M 7 16 L 16 16"></path>
            <path d="M 9.69 4 L 20 4"></path>
            <path d="M14 8h.01"></path>
            <path d="M18 8h.01"></path>
            <path d="m2 2 20 20"></path>
            <path d="M20 20H4a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2"></path>
            <path d="M6 8h.01"></path>
            <path d="M8 12h.01"></path>
        </svg>
    }
}
