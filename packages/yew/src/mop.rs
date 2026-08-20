use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct MopProps {
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
pub fn Mop(props: &MopProps) -> Html {
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
            <path d="M10 22c2.761 0 5-1.79 5-4-4.42 0-4.08-5-8.5-5a1 1 0 100 9za3 3 0 01-3-3" />
            <path d="M12.5 11.5 22 2" />
            <path
                d="m6.98 13.02 2.665-2.664a1.21 1.21 0 011.71 0l2.29 2.288a1.21 1.21 0 010 1.712l-2.088 2.087"
            />
        </svg>
    }
}
