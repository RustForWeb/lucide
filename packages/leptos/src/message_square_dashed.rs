use leptos::{prelude::*, svg::Svg};
#[component]
pub fn MessageSquareDashed(
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
            <path d="M12 19h.01" />
            <path d="M12 3h.01" />
            <path d="M16 19h.01" />
            <path d="M16 3h.01" />
            <path d="M2 13h.01" />
            <path d="M2 17v4.286a.71.71 0 0 0 1.212.502l2.202-2.202A2 2 0 0 1 6.828 19H8" />
            <path d="M2 5a2 2 0 0 1 2-2" />
            <path d="M2 9h.01" />
            <path d="M20 3a2 2 0 0 1 2 2" />
            <path d="M22 13h.01" />
            <path d="M22 17a2 2 0 0 1-2 2" />
            <path d="M22 9h.01" />
            <path d="M8 3h.01" />
        </svg>
    }
}
