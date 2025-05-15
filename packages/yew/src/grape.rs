use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct GrapeProps {
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
pub fn Grape(props: &GrapeProps) -> Html {
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
            <path d="M22 5V2l-5.89 5.89" />
            <circle cx="16.6" cy="15.89" r="3" />
            <circle cx="8.11" cy="7.4" r="3" />
            <circle cx="12.35" cy="11.65" r="3" />
            <circle cx="13.91" cy="5.85" r="3" />
            <circle cx="18.15" cy="10.09" r="3" />
            <circle cx="6.56" cy="13.2" r="3" />
            <circle cx="10.8" cy="17.44" r="3" />
            <circle cx="5" cy="19" r="3" />
        </svg>
    }
}
