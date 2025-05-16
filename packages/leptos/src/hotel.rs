use leptos::{prelude::*, svg::Svg};
#[component]
pub fn Hotel(
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
            <path d="M10 22v-6.57"></path>
            <path d="M12 11h.01"></path>
            <path d="M12 7h.01"></path>
            <path d="M14 15.43V22"></path>
            <path d="M15 16a5 5 0 0 0-6 0"></path>
            <path d="M16 11h.01"></path>
            <path d="M16 7h.01"></path>
            <path d="M8 11h.01"></path>
            <path d="M8 7h.01"></path>
            <rect x="4" y="2" width="16" height="20" rx="2"></rect>
        </svg>
    }
}
