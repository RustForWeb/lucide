use leptos::{prelude::*, svg::Svg};
#[component]
pub fn MonitorCog(
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
            <path d="M12 17v4"></path>
            <path d="m15.2 4.9-.9-.4"></path>
            <path d="m15.2 7.1-.9.4"></path>
            <path d="m16.9 3.2-.4-.9"></path>
            <path d="m16.9 8.8-.4.9"></path>
            <path d="m19.5 2.3-.4.9"></path>
            <path d="m19.5 9.7-.4-.9"></path>
            <path d="m21.7 4.5-.9.4"></path>
            <path d="m21.7 7.5-.9-.4"></path>
            <path d="M22 13v2a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h7"></path>
            <path d="M8 21h8"></path>
            <circle cx="18" cy="6" r="3"></circle>
        </svg>
    }
}
