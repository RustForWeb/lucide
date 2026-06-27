use leptos::{prelude::*, svg::Svg};
#[component]
pub fn WebcamOff(
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
            <path d="M12 22v-4" />
            <path d="M12.754 7.096a3 3 0 0 1 2.15 2.15" />
            <path d="M12.863 12.873a3 3 0 0 1-3.736-3.735" />
            <path d="M16.566 16.57A8 8 0 0 1 5.43 5.433" />
            <path d="m2 2 20 20" />
            <path d="M7 22h10" />
            <path d="M8.478 2.817a8 8 0 0 1 10.705 10.705" />
        </svg>
    }
}
