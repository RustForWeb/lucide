use leptos::{prelude::*, svg::Svg};
#[component]
pub fn FileCog(
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
            <path d="M15 8a1 1 0 0 1-1-1V2a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8z" />
            <path d="M20 8v12a2 2 0 0 1-2 2h-4.182" />
            <path d="m3.305 19.53.923-.382" />
            <path d="M4 10.592V4a2 2 0 0 1 2-2h8" />
            <path d="m4.228 16.852-.924-.383" />
            <path d="m5.852 15.228-.383-.923" />
            <path d="m5.852 20.772-.383.924" />
            <path d="m8.148 15.228.383-.923" />
            <path d="m8.53 21.696-.382-.924" />
            <path d="m9.773 16.852.922-.383" />
            <path d="m9.773 19.148.922.383" />
            <circle cx="7" cy="18" r="3" />
        </svg>
    }
}
