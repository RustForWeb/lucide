use leptos::{prelude::*, svg::Svg};
#[component]
pub fn Tractor(
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
            <path d="m10 11 11 .9a1 1 0 0 1 .8 1.1l-.665 4.158a1 1 0 0 1-.988.842H20" />
            <path d="M16 18h-5" />
            <path d="M18 5a1 1 0 0 0-1 1v5.573" />
            <path d="M3 4h8.129a1 1 0 0 1 .99.863L13 11.246" />
            <path d="M4 11V4" />
            <path d="M7 15h.01" />
            <path d="M8 10.1V4" />
            <circle cx="18" cy="18" r="2" />
            <circle cx="7" cy="15" r="5" />
        </svg>
    }
}
