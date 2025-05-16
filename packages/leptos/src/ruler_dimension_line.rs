use leptos::{prelude::*, svg::Svg};
#[component]
pub fn RulerDimensionLine(
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
            <path d="M12 15v-3.014"></path>
            <path d="M16 15v-3.014"></path>
            <path d="M20 6H4"></path>
            <path d="M20 8V4"></path>
            <path d="M4 8V4"></path>
            <path d="M8 15v-3.014"></path>
            <rect x="3" y="12" width="18" height="7" rx="1"></rect>
        </svg>
    }
}
