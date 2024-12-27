use leptos::{prelude::*, svg::Svg};
#[component]
pub fn BellElectric(
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
            <path d="M18.8 4A6.3 8.7 0 0 1 20 9" />
            <path d="M9 9h.01" />
            <circle cx="9" cy="9" r="7" />
            <rect width="10" height="6" x="4" y="16" rx="2" />
            <path d="M14 19c3 0 4.6-1.6 4.6-1.6" />
            <circle cx="20" cy="16" r="2" />
        </svg>
    }
}
