use leptos::{prelude::*, svg::Svg};
#[component]
pub fn Lighthouse(
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
            <path d="M12 3V2" />
            <path d="M16.066 16.865 7 22l2-11V6a3 3 0 016 0v5l2 11" />
            <path d="m19.792 4.5.866-.5" />
            <path d="m19.797 13.5.866.5" />
            <path d="M21 9h1" />
            <path d="M3 9H2" />
            <path d="m4.203 13.5-.866.5" />
            <path d="M4.208 4.5 3.342 4" />
            <path d="M5.5 22h13" />
            <path d="m7.932 16.875 7.377-4.178" />
            <path d="M8 11h8" />
            <path d="M8 7h8" />
        </svg>
    }
}
