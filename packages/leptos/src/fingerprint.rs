use leptos::{prelude::*, svg::Svg};
#[component]
pub fn Fingerprint(
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
            <path d="M12 10a2 2 0 0 0-2 2c0 1.02-.1 2.51-.26 4"></path>
            <path d="M14 13.12c0 2.38 0 6.38-1 8.88"></path>
            <path d="M17.29 21.02c.12-.6.43-2.3.5-3.02"></path>
            <path d="M2 12a10 10 0 0 1 18-6"></path>
            <path d="M2 16h.01"></path>
            <path d="M21.8 16c.2-2 .131-5.354 0-6"></path>
            <path d="M5 19.5C5.5 18 6 15 6 12a6 6 0 0 1 .34-2"></path>
            <path d="M8.65 22c.21-.66.45-1.32.57-2"></path>
            <path d="M9 6.8a6 6 0 0 1 9 5.2v2"></path>
        </svg>
    }
}
