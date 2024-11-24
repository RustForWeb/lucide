use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct SunSnowProps {
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
pub fn SunSnow(props: &SunSnowProps) -> Html {
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
            <path d="M10 9a3 3 0 1 0 0 6" />
            <path d="M2 12h1" />
            <path d="M14 21V3" />
            <path d="M10 4V3" />
            <path d="M10 21v-1" />
            <path d="m3.64 18.36.7-.7" />
            <path d="m4.34 6.34-.7-.7" />
            <path d="M14 12h8" />
            <path d="m17 4-3 3" />
            <path d="m14 17 3 3" />
            <path d="m21 15-3-3 3-3" />
        </svg>
    }
}