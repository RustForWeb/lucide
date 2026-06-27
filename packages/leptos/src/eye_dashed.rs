use leptos::{prelude::*, svg::Svg};
#[component]
pub fn EyeDashed(
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
            <path d="M13.054 18.946a11 11 0 0 1-2.11 0" />
            <path d="M13.054 5.054a11 11 0 0 0-2.11-.001" />
            <path d="M17.072 6.274a11 11 0 0 1 1.753 1.173" />
            <path d="M18.825 16.552a11 11 0 0 1-1.753 1.174" />
            <path d="M2.514 13.303a11 11 0 0 1-.452-.954 1 1 0 0 1 0-.697 11 11 0 0 1 .45-.955" />
            <path d="M21.485 10.697a11 11 0 0 1 .453.955 1 1 0 0 1 0 .697 11 11 0 0 1-.453.954" />
            <path d="M5.173 7.448a11 11 0 0 1 1.753-1.174" />
            <path d="M6.926 17.726a11 11 0 0 1-1.753-1.174" />
            <circle cx="12" cy="12" r="3" />
        </svg>
    }
}
