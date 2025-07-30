use leptos::{prelude::*, svg::Svg};
#[component]
pub fn UserStar(
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
            <path d="M8.5 15H7a4 4 0 0 0-4 4v2" />
            <path d="M16.5 12.903a.229.229 0 0 1 .41 0l.997 2.022a.92.92 0 0 0 .69.501l2.23.326a.229.229 0 0 1 .127.392l-1.613 1.57a.92.92 0 0 0-.264.812l.381 2.22a.229.229 0 0 1-.333.241L17.13 19.94a.92.92 0 0 0-.853 0l-1.993 1.048a.229.229 0 0 1-.333-.242l.38-2.22a.92.92 0 0 0-.264-.81l-1.613-1.571a.229.229 0 0 1 .127-.392l2.23-.326a.92.92 0 0 0 .69-.501z" />
            <circle cx="10" cy="7" r="4" />
        </svg>
    }
}
