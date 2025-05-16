use leptos::{prelude::*, svg::Svg};
#[component]
pub fn Columns3Cog(
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
            <path d="M10.5 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v5.5"></path>
            <path d="m14.3 19.6 1-.4"></path>
            <path d="M15 3v7.5"></path>
            <path d="m15.2 16.9-.9-.3"></path>
            <path d="m16.6 21.7.3-.9"></path>
            <path d="m16.8 15.3-.4-1"></path>
            <path d="m19.1 15.2.3-.9"></path>
            <path d="m19.6 21.7-.4-1"></path>
            <path d="m20.7 16.8 1-.4"></path>
            <path d="m21.7 19.4-.9-.3"></path>
            <path d="M9 3v18"></path>
            <circle cx="18" cy="18" r="3"></circle>
        </svg>
    }
}
