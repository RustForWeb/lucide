use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct SquareBottomDashedScissorsProps {
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
#[component]
pub fn SquareBottomDashedScissors(props: &SquareBottomDashedScissorsProps) -> Html {
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
            <line x1="5" y1="3" x2="19" y2="3" />
            <line x1="3" y1="5" x2="3" y2="19" />
            <line x1="21" y1="5" x2="21" y2="19" />
            <line x1="9" y1="21" x2="10" y2="21" />
            <line x1="14" y1="21" x2="15" y2="21" />
            <path d="M 3 5 A2 2 0 0 1 5 3" />
            <path d="M 19 3 A2 2 0 0 1 21 5" />
            <path d="M 5 21 A2 2 0 0 1 3 19" />
            <path d="M 21 19 A2 2 0 0 1 19 21" />
            <circle cx="8.5" cy="8.5" r="1.5" />
            <line x1="9.56066" y1="9.56066" x2="12" y2="12" />
            <line x1="17" y1="17" x2="14.82" y2="14.82" />
            <circle cx="8.5" cy="15.5" r="1.5" />
            <line x1="9.56066" y1="14.43934" x2="17" y2="7" />
        </svg>
    }
}
