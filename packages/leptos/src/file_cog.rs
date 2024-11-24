use leptos::{prelude::*, svg::Svg};
#[component]
pub fn FileCog(
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
            <path d="M14 2v4a2 2 0 0 0 2 2h4"></path>
            <path d="m3.2 12.9-.9-.4"></path>
            <path d="m3.2 15.1-.9.4"></path>
            <path d="M4.677 21.5a2 2 0 0 0 1.313.5H18a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v2.5"></path>
            <path d="m4.9 11.2-.4-.9"></path>
            <path d="m4.9 16.8-.4.9"></path>
            <path d="m7.5 10.3-.4.9"></path>
            <path d="m7.5 17.7-.4-.9"></path>
            <path d="m9.7 12.5-.9.4"></path>
            <path d="m9.7 15.5-.9-.4"></path>
            <circle cx="6" cy="14" r="3"></circle>
        </svg>
    }
}