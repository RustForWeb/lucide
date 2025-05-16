use leptos::{prelude::*, svg::Svg};
#[component]
pub fn UserRoundCog(
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
            <path d="m14.305 19.53.923-.382"></path>
            <path d="m15.228 16.852-.923-.383"></path>
            <path d="m16.852 15.228-.383-.923"></path>
            <path d="m16.852 20.772-.383.924"></path>
            <path d="m19.148 15.228.383-.923"></path>
            <path d="m19.53 21.696-.382-.924"></path>
            <path d="M2 21a8 8 0 0 1 10.434-7.62"></path>
            <path d="m20.772 16.852.924-.383"></path>
            <path d="m20.772 19.148.924.383"></path>
            <circle cx="10" cy="8" r="5"></circle>
            <circle cx="18" cy="18" r="3"></circle>
        </svg>
    }
}
