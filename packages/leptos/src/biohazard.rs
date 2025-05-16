use leptos::{prelude::*, svg::Svg};
#[component]
pub fn Biohazard(
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
            <circle cx="12" cy="11.9" r="2"></circle>
            <path d="M6.7 3.4c-.9 2.5 0 5.2 2.2 6.7C6.5 9 3.7 9.6 2 11.6"></path>
            <path d="m8.9 10.1 1.4.8"></path>
            <path d="M17.3 3.4c.9 2.5 0 5.2-2.2 6.7 2.4-1.2 5.2-.6 6.9 1.5"></path>
            <path d="m15.1 10.1-1.4.8"></path>
            <path d="M16.7 20.8c-2.6-.4-4.6-2.6-4.7-5.3-.2 2.6-2.1 4.8-4.7 5.2"></path>
            <path d="M12 13.9v1.6"></path>
            <path d="M13.5 5.4c-1-.2-2-.2-3 0"></path>
            <path d="M17 16.4c.7-.7 1.2-1.6 1.5-2.5"></path>
            <path d="M5.5 13.9c.3.9.8 1.8 1.5 2.5"></path>
        </svg>
    }
}
