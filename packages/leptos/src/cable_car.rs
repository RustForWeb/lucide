use leptos::{prelude::*, svg::Svg};
#[component]
pub fn CableCar(
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
            <path d="M10 3h.01"></path>
            <path d="M14 2h.01"></path>
            <path d="m2 9 20-5"></path>
            <path d="M12 12V6.5"></path>
            <rect width="16" height="10" x="4" y="12" rx="3"></rect>
            <path d="M9 12v5"></path>
            <path d="M15 12v5"></path>
            <path d="M4 17h16"></path>
        </svg>
    }
}
