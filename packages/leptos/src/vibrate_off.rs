use leptos::{prelude::*, svg::Svg};
#[component]
pub fn VibrateOff(
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
            <path d="m2 8 2 2-2 2 2 2-2 2"></path>
            <path d="m22 8-2 2 2 2-2 2 2 2"></path>
            <path d="M8 8v10c0 .55.45 1 1 1h6c.55 0 1-.45 1-1v-2"></path>
            <path d="M16 10.34V6c0-.55-.45-1-1-1h-4.34"></path>
            <line x1="2" x2="22" y1="2" y2="22"></line>
        </svg>
    }
}
