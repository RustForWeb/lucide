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
            <path d="M14 2v4a2 2 0 0 0 2 2h4"></path>
            <path d="m2.305 15.53.923-.382"></path>
            <path d="m3.228 12.852-.924-.383"></path>
            <path d="M4.677 21.5a2 2 0 0 0 1.313.5H18a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v2.5"></path>
            <path d="m4.852 11.228-.383-.923"></path>
            <path d="m4.852 16.772-.383.924"></path>
            <path d="m7.148 11.228.383-.923"></path>
            <path d="m7.53 17.696-.382-.924"></path>
            <path d="m8.772 12.852.923-.383"></path>
            <path d="m8.772 15.148.923.383"></path>
            <circle cx="6" cy="14" r="3"></circle>
        </svg>
    }
}
