use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct BookDashedProps {
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
pub fn BookDashed(props: &BookDashedProps) -> Html {
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
            <path d="M12 17h2" />
            <path d="M12 22h2" />
            <path d="M12 2h2" />
            <path d="M18 22h1a1 1 0 0 0 1-1" />
            <path d="M18 2h1a1 1 0 0 1 1 1v1" />
            <path d="M20 15v2h-2" />
            <path d="M20 8v3" />
            <path d="M4 11V9" />
            <path d="M4 19.5V15" />
            <path d="M4 5v-.5A2.5 2.5 0 0 1 6.5 2H8" />
            <path d="M8 22H6.5a1 1 0 0 1 0-5H8" />
        </svg>
    }
}