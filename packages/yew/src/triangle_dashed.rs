use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct TriangleDashedProps {
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
pub fn TriangleDashed(props: &TriangleDashedProps) -> Html {
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
            <path d="M10.17 4.193a2 2 0 0 1 3.666.013" />
            <path d="M14 21h2" />
            <path d="m15.874 7.743 1 1.732" />
            <path d="m18.849 12.952 1 1.732" />
            <path d="M21.824 18.18a2 2 0 0 1-1.835 2.824" />
            <path d="M4.024 21a2 2 0 0 1-1.839-2.839" />
            <path d="m5.136 12.952-1 1.732" />
            <path d="M8 21h2" />
            <path d="m8.102 7.743-1 1.732" />
        </svg>
    }
}
