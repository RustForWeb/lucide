use leptos::{prelude::*, svg::Svg};
#[component]
pub fn BookDashed(
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
            <path d="M12 17h1.5"></path>
            <path d="M12 22h1.5"></path>
            <path d="M12 2h1.5"></path>
            <path d="M17.5 22H19a1 1 0 0 0 1-1"></path>
            <path d="M17.5 2H19a1 1 0 0 1 1 1v1.5"></path>
            <path d="M20 14v3h-2.5"></path>
            <path d="M20 8.5V10"></path>
            <path d="M4 10V8.5"></path>
            <path d="M4 19.5V14"></path>
            <path d="M4 4.5A2.5 2.5 0 0 1 6.5 2H8"></path>
            <path d="M8 22H6.5a1 1 0 0 1 0-5H8"></path>
        </svg>
    }
}
