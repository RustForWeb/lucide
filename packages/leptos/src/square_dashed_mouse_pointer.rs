use leptos::{prelude::*, svg::Svg};
#[component]
pub fn SquareDashedMousePointer(
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
            <path d="M12.034 12.681a.498.498 0 0 1 .647-.647l9 3.5a.5.5 0 0 1-.033.943l-3.444 1.068a1 1 0 0 0-.66.66l-1.067 3.443a.5.5 0 0 1-.943.033z"></path>
            <path d="M5 3a2 2 0 0 0-2 2"></path>
            <path d="M19 3a2 2 0 0 1 2 2"></path>
            <path d="M5 21a2 2 0 0 1-2-2"></path>
            <path d="M9 3h1"></path>
            <path d="M9 21h2"></path>
            <path d="M14 3h1"></path>
            <path d="M3 9v1"></path>
            <path d="M21 9v2"></path>
            <path d="M3 14v1"></path>
        </svg>
    }
}
