use leptos::{prelude::*, svg::Svg};
#[component]
pub fn RadioTower(
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
            <path d="M4.9 16.1C1 12.2 1 5.8 4.9 1.9"></path>
            <path d="M7.8 4.7a6.14 6.14 0 0 0-.8 7.5"></path>
            <circle cx="12" cy="9" r="2"></circle>
            <path d="M16.2 4.8c2 2 2.26 5.11.8 7.47"></path>
            <path d="M19.1 1.9a9.96 9.96 0 0 1 0 14.1"></path>
            <path d="M9.5 18h5"></path>
            <path d="m8 22 4-11 4 11"></path>
        </svg>
    }
}
