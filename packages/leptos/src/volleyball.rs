use leptos::{prelude::*, svg::Svg};
#[component]
pub fn Volleyball(
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
            <path d="M11.1 7.1a16.55 16.55 0 0 1 10.9 4"></path>
            <path d="M12 12a12.6 12.6 0 0 1-8.7 5"></path>
            <path d="M16.8 13.6a16.55 16.55 0 0 1-9 7.5"></path>
            <path d="M20.7 17a12.8 12.8 0 0 0-8.7-5 13.3 13.3 0 0 1 0-10"></path>
            <path d="M6.3 3.8a16.55 16.55 0 0 0 1.9 11.5"></path>
            <circle cx="12" cy="12" r="10"></circle>
        </svg>
    }
}
