use leptos::{prelude::*, svg::Svg};
#[component]
pub fn ShieldCogCorner(
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
            <path d="M11 22c-3.806-1.45-7-3.966-7-9V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1v4" />
            <path d="M14.923 16.547 14 16.164" />
            <path d="m14.923 18.843-.923.383" />
            <path d="M16.547 14.923 16.164 14" />
            <path d="m16.547 20.467-.383.924" />
            <path d="m18.843 14.923.383-.923" />
            <path d="m19.225 21.391-.382-.924" />
            <path d="m20.467 16.547.923-.383" />
            <path d="m20.467 18.843.923.383" />
            <circle cx="17.695" cy="17.695" r="3" />
        </svg>
    }
}
