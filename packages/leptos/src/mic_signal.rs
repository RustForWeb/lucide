use leptos::{prelude::*, svg::Svg};
#[component]
pub fn MicSignal(
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
            <path d="M12 17v4" />
            <path d="M18 11a6 6 0 00-3-5.197" />
            <path d="M2 11a10 10 0 015-8.662" />
            <path d="M22 11a10 10 0 00-5-8.662" />
            <path d="M6 11a6 6 0 013-5.197" />
            <path d="M9 21h6" />
            <rect x="10" y="9" width="4" height="8" rx="2" />
        </svg>
    }
}
