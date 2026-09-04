use leptos::{prelude::*, svg::Svg};
#[component]
pub fn VirusOff(
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
            <path d="M10.01 10h.01" />
            <path d="M12 14.991h.01" />
            <path d="M12 22v-3" />
            <path d="M12 2v3" />
            <path d="M13 22h-2" />
            <path d="M13 2h-2" />
            <path d="m16.5 19.794-1-1.733" />
            <path d="m16.5 4.205-1 1.732" />
            <path d="M18.891 13.235a7 7 0 00-8.126-8.126" />
            <path d="m19.794 7.5-1.732 1" />
            <path d="M2 12h3" />
            <path d="M2 13v-2" />
            <path d="m2 2 20 20" />
            <path d="M22 12h-3" />
            <path d="M22 13v-2" />
            <path d="m4.206 16.5 1.732-1" />
            <path d="m4.206 7.5 1.732 1" />
            <path d="M7.05 7.05a7 7 0 009.9 9.9" />
            <path d="m7.5 19.794 1-1.733" />
            <path d="M9 12h.01" />
        </svg>
    }
}
