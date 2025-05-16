use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct CloudCogProps {
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
pub fn CloudCog(props: &CloudCogProps) -> Html {
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
            <path d="m10.852 19.772-.383.924" />
            <path d="m13.148 14.228.383-.923" />
            <path d="M13.148 19.772a3 3 0 1 0-2.296-5.544l-.383-.923" />
            <path d="m13.53 20.696-.382-.924a3 3 0 1 1-2.296-5.544" />
            <path d="m14.772 15.852.923-.383" />
            <path d="m14.772 18.148.923.383" />
            <path d="M4.2 15.1a7 7 0 1 1 9.93-9.858A7 7 0 0 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.2" />
            <path d="m9.228 15.852-.923-.383" />
            <path d="m9.228 18.148-.923.383" />
        </svg>
    }
}
