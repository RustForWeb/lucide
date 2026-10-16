use leptos::{prelude::*, svg::Svg};
#[component]
pub fn UsersCollaboration(
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
            <path d="M12.925 3.047a9 9 0 017.616 6.11" />
            <path d="M13 22a2 2 0 012-2h4a2 2 0 012 2" />
            <path d="M3 12a2 2 0 012-2h4a2 2 0 012 2" />
            <path d="M3.889 15.9a9 9 0 005.247 4.632" />
            <circle cx="17" cy="14" r="2" />
            <circle cx="7" cy="4" r="2" />
        </svg>
    }
}
