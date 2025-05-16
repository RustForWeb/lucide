use leptos::{prelude::*, svg::Svg};
#[component]
pub fn WifiOff(
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
            <path d="M12 20h.01"></path>
            <path d="M8.5 16.429a5 5 0 0 1 7 0"></path>
            <path d="M5 12.859a10 10 0 0 1 5.17-2.69"></path>
            <path d="M19 12.859a10 10 0 0 0-2.007-1.523"></path>
            <path d="M2 8.82a15 15 0 0 1 4.177-2.643"></path>
            <path d="M22 8.82a15 15 0 0 0-11.288-3.764"></path>
            <path d="m2 2 20 20"></path>
        </svg>
    }
}
