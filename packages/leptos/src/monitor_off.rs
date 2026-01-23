use leptos::{prelude::*, svg::Svg};
#[component]
pub fn MonitorOff(
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
            <path d="M12 17v4" />
            <path d="M17 17H4a2 2 0 0 1-2-2V5a2 2 0 0 1 1.184-1.826" />
            <path d="m2 2 20 20" />
            <path d="M8 21h8" />
            <path d="M8.656 3H20a2 2 0 0 1 2 2v10a2 2 0 0 1-.293 1.042" />
        </svg>
    }
}
