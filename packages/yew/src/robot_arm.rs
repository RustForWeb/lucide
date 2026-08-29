use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct RobotArmProps {
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
pub fn RobotArm(props: &RobotArmProps) -> Html {
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
            <path d="M12 21 7.5 8.322" />
            <path d="m14 7 1.75-3.767a.5.5 0 0 1 .662-.172L20 5.005" />
            <path d="m20 8.998-3.588 1.944a.5.5 0 0 1-.662-.172L14 7H8" />
            <path d="M3.486 21h10" />
            <path d="M5 21V8.732" />
            <circle cx="6" cy="7" r="2" />
        </svg>
    }
}
