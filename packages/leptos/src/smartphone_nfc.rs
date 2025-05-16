use leptos::{prelude::*, svg::Svg};
#[component]
pub fn SmartphoneNfc(
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
            <rect width="7" height="12" x="2" y="6" rx="1" />
            <path d="M13 8.32a7.43 7.43 0 0 1 0 7.36" />
            <path d="M16.46 6.21a11.76 11.76 0 0 1 0 11.58" />
            <path d="M19.91 4.1a15.91 15.91 0 0 1 .01 15.8" />
        </svg>
    }
}
