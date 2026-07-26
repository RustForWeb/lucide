use leptos::{prelude::*, svg::Svg};
#[component]
pub fn MicAudioLines(
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
            <path d="M10 3v2.341" />
            <path d="M12 17v4" />
            <path d="M14 5v.341" />
            <path d="M18 5v13" />
            <path d="M2 10v3" />
            <path d="M22 10v3" />
            <path d="M6 6v11" />
            <path d="M9 21h6" />
            <rect width="4" height="8" x="10" y="9" rx="2" />
        </svg>
    }
}
