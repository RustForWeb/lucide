use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct CircleDashedProps {
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
    pub style: std::option::Option<AttrValue>,
    #[prop_or_default]
    pub node_ref: NodeRef,
}
#[function_component]
pub fn CircleDashed(props: &CircleDashedProps) -> Html {
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
            style={props.style.clone()}
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
            <path d="M10.1 2.182a10 10 0 0 1 3.8 0" />
            <path d="M13.9 21.818a10 10 0 0 1-3.8 0" />
            <path d="M17.609 3.721a10 10 0 0 1 2.69 2.7" />
            <path d="M2.182 13.9a10 10 0 0 1 0-3.8" />
            <path d="M20.279 17.609a10 10 0 0 1-2.7 2.69" />
            <path d="M21.818 10.1a10 10 0 0 1 0 3.8" />
            <path d="M3.721 6.391a10 10 0 0 1 2.7-2.69" />
            <path d="M6.391 20.279a10 10 0 0 1-2.69-2.7" />
        </svg>
    }
}
