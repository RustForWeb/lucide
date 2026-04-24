use leptos::{prelude::*, svg::Svg};
#[component]
pub fn RadioOff(
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
            <path d="M10.4103 10.7852C10.1529 11.1218 10 11.5425 10 11.999C10 13.1036 10.8954 13.999 12 13.999C12.5077 13.999 12.9713 13.8098 13.324 13.498" />
            <path d="M16.1992 7.80078C17.4739 9.07549 18.0422 10.8109 17.9039 12.5134" />
            <path d="M19.0996 4.89844C22.0892 7.88804 22.7871 12.2879 21.1932 15.936" />
            <path d="M2 2L22 22" />
            <path d="M4.89961 19.0984C0.999609 15.1984 0.999609 8.79844 4.89961 4.89844" />
            <path d="M7.79922 16.1992C5.66828 14.0683 5.51165 10.6498 7.32931 8.25" />
        </svg>
    }
}
