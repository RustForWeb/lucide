use leptos::{prelude::*, svg::Svg};
#[component]
pub fn UserRoundCog(
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
            <path d="M2 21a8 8 0 0 1 10.434-7.62" />
            <circle cx="10" cy="8" r="5" />
            <circle cx="18" cy="18" r="3" />
            <path d="m19.5 14.3-.4.9" />
            <path d="m16.9 20.8-.4.9" />
            <path d="m21.7 19.5-.9-.4" />
            <path d="m15.2 16.9-.9-.4" />
            <path d="m21.7 16.5-.9.4" />
            <path d="m15.2 19.1-.9.4" />
            <path d="m19.5 21.7-.4-.9" />
            <path d="m16.9 15.2-.4-.9" />
        </svg>
    }
}
