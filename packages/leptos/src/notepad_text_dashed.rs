use leptos::{prelude::*, svg::Svg};
#[component]
pub fn NotepadTextDashed(
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
            <path d="M8 2v4" />
            <path d="M12 2v4" />
            <path d="M16 2v4" />
            <path d="M16 4h2a2 2 0 0 1 2 2v2" />
            <path d="M20 12v2" />
            <path d="M20 18v2a2 2 0 0 1-2 2h-1" />
            <path d="M13 22h-2" />
            <path d="M7 22H6a2 2 0 0 1-2-2v-2" />
            <path d="M4 14v-2" />
            <path d="M4 8V6a2 2 0 0 1 2-2h2" />
            <path d="M8 10h6" />
            <path d="M8 14h8" />
            <path d="M8 18h5" />
        </svg>
    }
}
