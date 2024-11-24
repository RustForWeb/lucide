use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct SnowflakeProps {
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
pub fn Snowflake(props: &SnowflakeProps) -> Html {
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
            <line x1="2" x2="22" y1="12" y2="12" />
            <line x1="12" x2="12" y1="2" y2="22" />
            <path d="m20 16-4-4 4-4" />
            <path d="m4 8 4 4-4 4" />
            <path d="m16 4-4 4-4-4" />
            <path d="m8 20 4-4 4 4" />
        </svg>
    }
}
