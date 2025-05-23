use leptos::{prelude::*, svg::Svg};
#[component]
pub fn ChartScatter(
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
            <circle cx="7.5" cy="7.5" r=".5" fill="currentColor" />
            <circle cx="18.5" cy="5.5" r=".5" fill="currentColor" />
            <circle cx="11.5" cy="11.5" r=".5" fill="currentColor" />
            <circle cx="7.5" cy="16.5" r=".5" fill="currentColor" />
            <circle cx="17.5" cy="14.5" r=".5" fill="currentColor" />
            <path d="M3 3v16a2 2 0 0 0 2 2h16" />
        </svg>
    }
}
