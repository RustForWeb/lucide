use leptos::{prelude::*, svg::Svg};
#[component]
pub fn BugPlay(
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
            <path d="M12.765 21.522a.5.5 0 0 1-.765-.424v-8.196a.5.5 0 0 1 .765-.424l5.878 3.674a1 1 0 0 1 0 1.696z" />
            <path d="M14.12 3.88 16 2" />
            <path d="M18 11a4 4 0 0 0-4-4h-4a4 4 0 0 0-4 4v3a6.1 6.1 0 0 0 2 4.5" />
            <path d="M20.97 5c0 2.1-1.6 3.8-3.5 4" />
            <path d="M3 21c0-2.1 1.7-3.9 3.8-4" />
            <path d="M6 13H2" />
            <path d="M6.53 9C4.6 8.8 3 7.1 3 5" />
            <path d="m8 2 1.88 1.88" />
            <path d="M9 7.13v-1a3.003 3.003 0 1 1 6 0v1" />
        </svg>
    }
}
