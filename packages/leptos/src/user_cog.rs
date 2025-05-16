use leptos::{prelude::*, svg::Svg};
#[component]
pub fn UserCog(
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
            <path d="M10 15H6a4 4 0 0 0-4 4v2" />
            <path d="m14.305 16.53.923-.382" />
            <path d="m15.228 13.852-.923-.383" />
            <path d="m16.852 12.228-.383-.923" />
            <path d="m16.852 17.772-.383.924" />
            <path d="m19.148 12.228.383-.923" />
            <path d="m19.53 18.696-.382-.924" />
            <path d="m20.772 13.852.924-.383" />
            <path d="m20.772 16.148.924.383" />
            <circle cx="18" cy="15" r="3" />
            <circle cx="9" cy="7" r="4" />
        </svg>
    }
}
