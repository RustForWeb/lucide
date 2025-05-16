use leptos::{prelude::*, svg::Svg};
#[component]
pub fn BriefcaseConveyorBelt(
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
            <path d="M10 20v2"></path>
            <path d="M14 20v2"></path>
            <path d="M18 20v2"></path>
            <path d="M21 20H3"></path>
            <path d="M6 20v2"></path>
            <path d="M8 16V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v12"></path>
            <rect x="4" y="6" width="16" height="10" rx="2"></rect>
        </svg>
    }
}
