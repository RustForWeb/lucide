use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct SlackProps {
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
pub fn Slack(props: &SlackProps) -> Html {
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
            <rect width="3" height="8" x="13" y="2" rx="1.5" />
            <path d="M19 8.5V10h1.5A1.5 1.5 0 1 0 19 8.5" />
            <rect width="3" height="8" x="8" y="14" rx="1.5" />
            <path d="M5 15.5V14H3.5A1.5 1.5 0 1 0 5 15.5" />
            <rect width="8" height="3" x="14" y="13" rx="1.5" />
            <path d="M15.5 19H14v1.5a1.5 1.5 0 1 0 1.5-1.5" />
            <rect width="8" height="3" x="2" y="8" rx="1.5" />
            <path d="M8.5 5H10V3.5A1.5 1.5 0 1 0 8.5 5" />
        </svg>
    }
}
