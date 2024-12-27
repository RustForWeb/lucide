use leptos::{prelude::*, svg::Svg};
#[component]
pub fn FolderCog(
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
            <circle cx="18" cy="18" r="3" />
            <path d="M10.3 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v3.3" />
            <path d="m21.7 19.4-.9-.3" />
            <path d="m15.2 16.9-.9-.3" />
            <path d="m16.6 21.7.3-.9" />
            <path d="m19.1 15.2.3-.9" />
            <path d="m19.6 21.7-.4-1" />
            <path d="m16.8 15.3-.4-1" />
            <path d="m14.3 19.6 1-.4" />
            <path d="m20.7 16.8 1-.4" />
        </svg>
    }
}
