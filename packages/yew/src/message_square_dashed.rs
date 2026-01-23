use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct MessageSquareDashedProps {
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
pub fn MessageSquareDashed(props: &MessageSquareDashedProps) -> Html {
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
            <path d="M14 3h2" />
            <path d="M16 19h-2" />
            <path d="M2 12v-2" />
            <path d="M2 16v5.286a.71.71 0 0 0 1.212.502l1.149-1.149" />
            <path d="M20 19a2 2 0 0 0 2-2v-1" />
            <path d="M22 10v2" />
            <path d="M22 6V5a2 2 0 0 0-2-2" />
            <path d="M4 3a2 2 0 0 0-2 2v1" />
            <path d="M8 19h2" />
            <path d="M8 3h2" />
        </svg>
    }
}
