use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct TicketsPlaneProps {
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
pub fn TicketsPlane(props: &TicketsPlaneProps) -> Html {
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
            <path d="M10.5 17h1.227a2 2 0 0 0 1.345-.52L18 12" />
            <path d="m12 13.5 3.75.5" />
            <path d="m4.5 8 10.58-5.06a1 1 0 0 1 1.342.488L18.5 8" />
            <path d="M6 10V8" />
            <path d="M6 14v1" />
            <path d="M6 19v2" />
            <rect x="2" y="8" width="20" height="13" rx="2" />
        </svg>
    }
}
