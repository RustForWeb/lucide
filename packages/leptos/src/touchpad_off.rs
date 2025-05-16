use leptos::{prelude::*, svg::Svg};
#[component]
pub fn TouchpadOff(
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
            <path d="M12 20v-6" />
            <path d="M19.656 14H22" />
            <path d="M2 14h12" />
            <path d="m2 2 20 20" />
            <path d="M20 20H4a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2" />
            <path d="M9.656 4H20a2 2 0 0 1 2 2v10.344" />
        </svg>
    }
}
