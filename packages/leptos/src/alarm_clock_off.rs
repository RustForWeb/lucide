use leptos::{prelude::*, svg::Svg};
#[component]
pub fn AlarmClockOff(
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
            <path d="M6.87 6.87a8 8 0 1 0 11.26 11.26" />
            <path d="M19.9 14.25a8 8 0 0 0-9.15-9.15" />
            <path d="m22 6-3-3" />
            <path d="M6.26 18.67 4 21" />
            <path d="m2 2 20 20" />
            <path d="M4 4 2 6" />
        </svg>
    }
}
