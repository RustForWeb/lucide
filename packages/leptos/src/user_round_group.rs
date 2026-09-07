use leptos::{prelude::*, svg::Svg};
#[component]
pub fn UserRoundGroup(
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
            <path d="M17 21a5 5 0 00-10 0" />
            <path d="M22 10.5a3.5 3.5 0 00-5.507-2.868" />
            <path d="M7.507 7.632A3.5 3.5 0 002 10.5" />
            <circle cx="12" cy="13" r="3" />
            <circle cx="18.5" cy="4.5" r="2.5" />
            <circle cx="5.5" cy="4.5" r="2.5" />
        </svg>
    }
}
