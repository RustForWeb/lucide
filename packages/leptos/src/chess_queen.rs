use leptos::{prelude::*, svg::Svg};
#[component]
pub fn ChessQueen(
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
            <path d="m10.68 5.499-.996 4.982a.65.65 0 01-1.131.295L5.885 7.663" />
            <path d="m13.32 5.5.995 4.98a.65.65 0 001.131.296l2.668-3.113" />
            <path d="M20.428 8.952 17.5 18" />
            <path d="M4 20a2 2 0 012-2h12a2 2 0 012 2v1a1 1 0 01-1 1H5a1 1 0 01-1-1z" />
            <path d="M6.5 18 3.571 8.952" />
            <circle cx="12" cy="4" r="2" />
            <circle cx="20" cy="7" r="2" />
            <circle cx="4" cy="7" r="2" />
        </svg>
    }
}
