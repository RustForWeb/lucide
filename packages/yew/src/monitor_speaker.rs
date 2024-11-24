use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct MonitorSpeakerProps {
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
pub fn MonitorSpeaker(props: &MonitorSpeakerProps) -> Html {
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
            <path d="M5.5 20H8" />
            <path d="M17 9h.01" />
            <rect width="10" height="16" x="12" y="4" rx="2" />
            <path d="M8 6H4a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h4" />
            <circle cx="17" cy="15" r="1" />
        </svg>
    }
}