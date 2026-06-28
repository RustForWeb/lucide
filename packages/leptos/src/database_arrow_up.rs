use leptos::{prelude::*, svg::Svg};
#[component]
pub fn DatabaseArrowUp(
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
            <path d="M19 22v-6" />
            <path d="M21 12.536V5" />
            <path d="m22 19-3-3-3 3" />
            <path d="M3 12A9 3 0 0 0 14.457 14.886" />
            <path d="M3 5V19A9 3 0 0 0 13.318 21.968" />
            <ellipse cx="12" cy="5" rx="9" ry="3" />
        </svg>
    }
}
