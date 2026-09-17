use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct MessagesCircleProps {
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
#[component]
pub fn MessagesCircle(props: &MessagesCircleProps) -> Html {
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
            <path
                d="M19.95 10.05a7 7 0 011.412 7.872 1 1 0 00-.058.787l.675 2.089a1 1 0 01-1.236 1.168l-2.155-.631a1 1 0 00-.745.06 7 7 0 01-7.793-1.445"
            />
            <path
                d="M2.696 12.708a1 1 0 00-.058-.785 7 7 0 113.518 3.473 1 1 0 00-.744-.061l-2.155.63a1 1 0 01-1.236-1.167z"
            />
        </svg>
    }
}
