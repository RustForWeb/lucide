use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct UsbProps {
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
pub fn Usb(props: &UsbProps) -> Html {
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
            <circle cx="10" cy="7" r="1" />
            <circle cx="4" cy="20" r="1" />
            <path d="M4.7 19.3 19 5" />
            <path d="m21 3-3 1 2 2Z" />
            <path d="M9.26 7.68 5 12l2 5" />
            <path d="m10 14 5 2 3.5-3.5" />
            <path d="m18 12 1-1 1 1-1 1Z" />
        </svg>
    }
}
