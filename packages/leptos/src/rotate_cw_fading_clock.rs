use leptos::{prelude::*, svg::Svg};
#[component]
pub fn RotateCwFadingClock(
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
            <path d="M12 3a9.75 9.75 0 0 1 6.74 2.74" />
            <path d="M18.74 5.74 21 8" />
            <path d="M21 8V3" />
            <path d="M7.5 19.794c-6-3.464-6-12.124 0-15.588" />
            <path d="M7.5 4.206A9 9 0 0 1 12 3" />
            <path d="M12 7v5l4 2" />
            <path d="M14 20.775A9 9 0 0 1 12 21" />
            <path d="M19 17.656a9 9 0 0 1-1.5 1.456" />
            <path d="M21 12a9 9 0 0 1-.228 2" />
            <path d="M21 8h-5" />
        </svg>
    }
}
