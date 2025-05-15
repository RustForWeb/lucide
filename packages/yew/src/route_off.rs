use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct RouteOffProps {
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
pub fn RouteOff(props: &RouteOffProps) -> Html {
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
            <circle cx="6" cy="19" r="3" />
            <path d="M9 19h8.5c.4 0 .9-.1 1.3-.2" />
            <path d="M5.2 5.2A3.5 3.53 0 0 0 6.5 12H12" />
            <path d="m2 2 20 20" />
            <path d="M21 15.3a3.5 3.5 0 0 0-3.3-3.3" />
            <path d="M15 5h-4.3" />
            <circle cx="18" cy="5" r="3" />
        </svg>
    }
}
