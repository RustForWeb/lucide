use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct LighthouseProps {
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
pub fn Lighthouse(props: &LighthouseProps) -> Html {
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
            <path d="M12 3V2" />
            <path d="M16.066 16.865 7 22l2-11V6a3 3 0 016 0v5l2 11" />
            <path d="m19.792 4.5.866-.5" />
            <path d="m19.797 13.5.866.5" />
            <path d="M21 9h1" />
            <path d="M3 9H2" />
            <path d="m4.203 13.5-.866.5" />
            <path d="M4.208 4.5 3.342 4" />
            <path d="M5.5 22h13" />
            <path d="m7.932 16.875 7.377-4.178" />
            <path d="M8 11h8" />
            <path d="M8 7h8" />
        </svg>
    }
}
