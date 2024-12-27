use leptos::{prelude::*, svg::Svg};
#[component]
pub fn Vault(
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
            <rect width="18" height="18" x="3" y="3" rx="2" />
            <circle cx="7.5" cy="7.5" r=".5" fill="currentColor" />
            <path d="m7.9 7.9 2.7 2.7" />
            <circle cx="16.5" cy="7.5" r=".5" fill="currentColor" />
            <path d="m13.4 10.6 2.7-2.7" />
            <circle cx="7.5" cy="16.5" r=".5" fill="currentColor" />
            <path d="m7.9 16.1 2.7-2.7" />
            <circle cx="16.5" cy="16.5" r=".5" fill="currentColor" />
            <path d="m13.4 13.4 2.7 2.7" />
            <circle cx="12" cy="12" r="2" />
        </svg>
    }
}
