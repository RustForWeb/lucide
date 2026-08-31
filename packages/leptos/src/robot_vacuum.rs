use leptos::{prelude::*, svg::Svg};
#[component]
pub fn RobotVacuum(
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
            <path d="M11 17h2" />
            <path d="M12 12h.01" />
            <path d="M17 12a5 5 0 00-10 0" />
            <path d="M19 2v2.8" />
            <path d="M2 5h2.8" />
            <path d="M22 5h-2.8" />
            <path d="M5 2v2.8" />
            <circle cx="12" cy="12" r="10" />
        </svg>
    }
}
