use leptos::{prelude::*, svg::Svg};
#[component]
pub fn CloudCog(
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
            <path d="m10.852 19.772-.383.924"></path>
            <path d="m13.148 14.228.383-.923"></path>
            <path d="M13.148 19.772a3 3 0 1 0-2.296-5.544l-.383-.923"></path>
            <path d="m13.53 20.696-.382-.924a3 3 0 1 1-2.296-5.544"></path>
            <path d="m14.772 15.852.923-.383"></path>
            <path d="m14.772 18.148.923.383"></path>
            <path d="M4.2 15.1a7 7 0 1 1 9.93-9.858A7 7 0 0 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.2"></path>
            <path d="m9.228 15.852-.923-.383"></path>
            <path d="m9.228 18.148-.923.383"></path>
        </svg>
    }
}
