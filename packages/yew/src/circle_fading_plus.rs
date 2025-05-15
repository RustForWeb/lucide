use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct CircleFadingPlusProps {
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
pub fn CircleFadingPlus(props: &CircleFadingPlusProps) -> Html {
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
            <path d="M12 2a10 10 0 0 1 7.38 16.75" />
            <path d="M12 8v8" />
            <path d="M16 12H8" />
            <path d="M2.5 8.875a10 10 0 0 0-.5 3" />
            <path d="M2.83 16a10 10 0 0 0 2.43 3.4" />
            <path d="M4.636 5.235a10 10 0 0 1 .891-.857" />
            <path d="M8.644 21.42a10 10 0 0 0 7.631-.38" />
        </svg>
    }
}
