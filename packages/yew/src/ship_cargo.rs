use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct ShipCargoProps {
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
pub fn ShipCargo(props: &ShipCargoProps) -> Html {
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
            <path d="M12 15v-3" />
            <path d="M12 2v2" />
            <path
                d="M16.5 12V9a1 1 0 011-1h1a1 1 0 001-1V5a1 1 0 00-1-1h-13a1 1 0 00-1 1v2a1 1 0 001 1h1a1 1 0 011 1v3"
            />
            <path
                d="M19.38 19c1.076-1.815 1.636-4.89 1.628-6.008a1 1 0 00-1-.992H3.984a1 1 0 00-1 .984c-.03 1.86.97 5.621 2.826 7.776"
            />
            <path
                d="M2 20c.6.5 1.2 1 2.5 1 2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1s1.2 1 2.5 1c2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1"
            />
        </svg>
    }
}
