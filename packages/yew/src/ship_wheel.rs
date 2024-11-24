use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct ShipWheelProps {
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
pub fn ShipWheel(props: &ShipWheelProps) -> Html {
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
            <circle cx="12" cy="12" r="8" />
            <path d="M12 2v7.5" />
            <path d="m19 5-5.23 5.23" />
            <path d="M22 12h-7.5" />
            <path d="m19 19-5.23-5.23" />
            <path d="M12 14.5V22" />
            <path d="M10.23 13.77 5 19" />
            <path d="M9.5 12H2" />
            <path d="M10.23 10.23 5 5" />
            <circle cx="12" cy="12" r="2.5" />
        </svg>
    }
}