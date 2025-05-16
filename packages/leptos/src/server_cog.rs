use leptos::{prelude::*, svg::Svg};
#[component]
pub fn ServerCog(
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
            <path d="m10.852 14.772-.383.923" />
            <path d="M13.148 14.772a3 3 0 1 0-2.296-5.544l-.383-.923" />
            <path d="m13.148 9.228.383-.923" />
            <path d="m13.53 15.696-.382-.924a3 3 0 1 1-2.296-5.544" />
            <path d="m14.772 10.852.923-.383" />
            <path d="m14.772 13.148.923.383" />
            <path d="M4.5 10H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2h-.5" />
            <path d="M4.5 14H4a2 2 0 0 0-2 2v4a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-4a2 2 0 0 0-2-2h-.5" />
            <path d="M6 18h.01" />
            <path d="M6 6h.01" />
            <path d="m9.228 10.852-.923-.383" />
            <path d="m9.228 13.148-.923.383" />
        </svg>
    }
}
