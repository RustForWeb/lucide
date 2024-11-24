use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct CombineProps {
    #[prop_or(24)]
    pub size: usize,
    #[prop_or(AttrValue::from("currentColor"))]
    pub color: AttrValue,
    #[prop_or(AttrValue::from("none"))]
    pub fill: AttrValue,
    #[prop_or(2)]
    pub stroke_width: usize,
    #[prop_or(false)]
    pub absolute_stroke_width: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub node_ref: NodeRef,
}
#[function_component]
pub fn Combine(props: &CombineProps) -> Html {
    let stroke_width = if props.absolute_stroke_width {
        props.stroke_width * 24 / props.size
    } else {
        props.stroke_width
    };
    html! {
        <svg
            ref={props.node_ref.clone()}
            class={classes!("lucide", props.class
        .clone())}
            xmlns="http://www.w3.org/2000/svg"
            width={props.size.to_string()}
            height={props.size.to_string()}
            viewBox="0 0 24 24"
            fill={& props.fill}
            stroke={& props.color}
            stroke-width={stroke_width.to_string()}
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="M10 18H5a3 3 0 0 1-3-3v-1" />
            <path d="M14 2a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2" />
            <path d="M20 2a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2" />
            <path d="m7 21 3-3-3-3" />
            <rect x="14" y="14" width="8" height="8" rx="2" />
            <rect x="2" y="2" width="8" height="8" rx="2" />
        </svg>
    }
}