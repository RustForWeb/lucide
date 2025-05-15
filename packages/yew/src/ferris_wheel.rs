use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct FerrisWheelProps {
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
pub fn FerrisWheel(props: &FerrisWheelProps) -> Html {
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
            <circle cx="12" cy="12" r="2" />
            <path d="M12 2v4" />
            <path d="m6.8 15-3.5 2" />
            <path d="m20.7 7-3.5 2" />
            <path d="M6.8 9 3.3 7" />
            <path d="m20.7 17-3.5-2" />
            <path d="m9 22 3-8 3 8" />
            <path d="M8 22h8" />
            <path d="M18 18.7a9 9 0 1 0-12 0" />
        </svg>
    }
}
