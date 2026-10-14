use leptos::{prelude::*, svg::Svg};
#[component]
pub fn UsersRoundCollaboration(
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
            <path d="M20.453 8.911a9 9 0 00-6.696-5.738" />
            <path d="M21 22a4 4 0 00-8 0" />
            <path d="M3 12a4 4 0 017.963-.543" />
            <path d="M3.889 15.9a9 9 0 005.247 4.632" />
            <circle cx="17" cy="15" r="3" />
            <circle cx="7" cy="5" r="3" />
        </svg>
    }
}
