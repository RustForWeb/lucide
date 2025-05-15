use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct MessageCircleDashedProps {
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
pub fn MessageCircleDashed(props: &MessageCircleDashedProps) -> Html {
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
            <path d="M13.5 3.1c-.5 0-1-.1-1.5-.1s-1 .1-1.5.1" />
            <path d="M19.3 6.8a10.45 10.45 0 0 0-2.1-2.1" />
            <path d="M20.9 13.5c.1-.5.1-1 .1-1.5s-.1-1-.1-1.5" />
            <path d="M17.2 19.3a10.45 10.45 0 0 0 2.1-2.1" />
            <path d="M10.5 20.9c.5.1 1 .1 1.5.1s1-.1 1.5-.1" />
            <path d="M3.5 17.5 2 22l4.5-1.5" />
            <path d="M3.1 10.5c0 .5-.1 1-.1 1.5s.1 1 .1 1.5" />
            <path d="M6.8 4.7a10.45 10.45 0 0 0-2.1 2.1" />
        </svg>
    }
}
