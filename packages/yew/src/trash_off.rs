use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct TrashOffProps {
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
pub fn TrashOff(props: &TrashOffProps) -> Html {
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
            <path d="M10 11v6" />
            <path d="M14 17v-3" />
            <path d="M16 6V4a2 2 0 00-2-2h-4a2 2 0 00-1.576.768" />
            <path d="M19 6v7.344" />
            <path d="m2 2 20 20" />
            <path d="M21 6h-9.344" />
            <path d="M3 6h3" />
            <path d="M5 6v14a2 2 0 002 2h10a2 2 0 002-2v-1" />
        </svg>
    }
}
