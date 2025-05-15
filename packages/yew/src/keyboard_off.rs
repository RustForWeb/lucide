use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct KeyboardOffProps {
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
pub fn KeyboardOff(props: &KeyboardOffProps) -> Html {
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
            <path d="M 20 4 A2 2 0 0 1 22 6" />
            <path d="M 22 6 L 22 16.41" />
            <path d="M 7 16 L 16 16" />
            <path d="M 9.69 4 L 20 4" />
            <path d="M14 8h.01" />
            <path d="M18 8h.01" />
            <path d="m2 2 20 20" />
            <path d="M20 20H4a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2" />
            <path d="M6 8h.01" />
            <path d="M8 12h.01" />
        </svg>
    }
}
