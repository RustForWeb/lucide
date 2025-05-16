use leptos::{prelude::*, svg::Svg};
#[component]
pub fn Haze(
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
            <path d="m5.2 6.2 1.4 1.4"></path>
            <path d="M2 13h2"></path>
            <path d="M20 13h2"></path>
            <path d="m17.4 7.6 1.4-1.4"></path>
            <path d="M22 17H2"></path>
            <path d="M22 21H2"></path>
            <path d="M16 13a4 4 0 0 0-8 0"></path>
            <path d="M12 5V2.5"></path>
        </svg>
    }
}
