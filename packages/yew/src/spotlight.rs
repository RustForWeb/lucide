use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct SpotlightProps {
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
pub fn Spotlight(props: &SpotlightProps) -> Html {
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
            <path d="M15.295 19.562 16 22" />
            <path d="m17 16 3.758 2.098" />
            <path d="m19 12.5 3.026-.598" />
            <path
                d="M7.61 6.3a3 3 0 0 0-3.92 1.3l-1.38 2.79a3 3 0 0 0 1.3 3.91l6.89 3.597a1 1 0 0 0 1.342-.447l3.106-6.211a1 1 0 0 0-.447-1.341z"
            />
            <path d="M8 9V2" />
        </svg>
    }
}
