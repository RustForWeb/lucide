use leptos::{prelude::*, svg::Svg};
#[component]
pub fn CalendarCog(
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
            <path d="m15.2 16.9-.9-.4" />
            <path d="m15.2 19.1-.9.4" />
            <path d="M16 2v4" />
            <path d="m16.9 15.2-.4-.9" />
            <path d="m16.9 20.8-.4.9" />
            <path d="m19.5 14.3-.4.9" />
            <path d="m19.5 21.7-.4-.9" />
            <path d="M21 10.5V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h6" />
            <path d="m21.7 16.5-.9.4" />
            <path d="m21.7 19.5-.9-.4" />
            <path d="M3 10h18" />
            <path d="M8 2v4" />
            <circle cx="18" cy="18" r="3" />
        </svg>
    }
}
