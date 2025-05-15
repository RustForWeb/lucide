use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct GitBranchPlusProps {
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
pub fn GitBranchPlus(props: &GitBranchPlusProps) -> Html {
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
            <path d="M6 3v12" />
            <path d="M18 9a3 3 0 1 0 0-6 3 3 0 0 0 0 6z" />
            <path d="M6 21a3 3 0 1 0 0-6 3 3 0 0 0 0 6z" />
            <path d="M15 6a9 9 0 0 0-9 9" />
            <path d="M18 15v6" />
            <path d="M21 18h-6" />
        </svg>
    }
}
