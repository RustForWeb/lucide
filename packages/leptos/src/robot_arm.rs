use leptos::{prelude::*, svg::Svg};
#[component]
pub fn RobotArm(
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
            <path d="M12 21 7.5 8.322" />
            <path d="m14 7 1.75-3.767a.5.5 0 0 1 .662-.172L20 5.005" />
            <path d="m20 8.998-3.588 1.944a.5.5 0 0 1-.662-.172L14 7H8" />
            <path d="M3.486 21h10" />
            <path d="M5 21V8.732" />
            <circle cx="6" cy="7" r="2" />
        </svg>
    }
}
