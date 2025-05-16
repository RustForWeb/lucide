use leptos::{prelude::*, svg::Svg};
#[component]
pub fn Grape(
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
            <path d="M22 5V2l-5.89 5.89" />
            <circle cx="16.6" cy="15.89" r="3" />
            <circle cx="8.11" cy="7.4" r="3" />
            <circle cx="12.35" cy="11.65" r="3" />
            <circle cx="13.91" cy="5.85" r="3" />
            <circle cx="18.15" cy="10.09" r="3" />
            <circle cx="6.56" cy="13.2" r="3" />
            <circle cx="10.8" cy="17.44" r="3" />
            <circle cx="5" cy="19" r="3" />
        </svg>
    }
}
