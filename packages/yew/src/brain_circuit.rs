use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct BrainCircuitProps {
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
pub fn BrainCircuit(props: &BrainCircuitProps) -> Html {
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
            <path d="M12 13h4" />
            <path d="M12 18h6a2 2 0 012 2v1" />
            <path d="M12 8h8" />
            <path d="M12 9a4.17 4.17 0 01-3 4" />
            <path d="M16 8V5a2 2 0 012-2" />
            <path d="M6 18a4 4 0 01-2-7.465" />
            <path d="M6.004 5.125a4 4 0 00-2.526 5.77" />
            <path d="M6.294 6.294A3 3 0 1112 5v13a4 4 0 11-7.967-.518" />
            <circle cx="16" cy="13" r="0.5" fill="currentColor" />
            <circle cx="18" cy="3" r="0.5" fill="currentColor" />
            <circle cx="20" cy="21" r="0.5" fill="currentColor" />
            <circle cx="20" cy="8" r="0.5" fill="currentColor" />
        </svg>
    }
}
