use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct ChartScatterProps {
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
pub fn ChartScatter(props: &ChartScatterProps) -> Html {
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
            <circle cx="7.5" cy="7.5" r=".5" fill="currentColor" />
            <circle cx="18.5" cy="5.5" r=".5" fill="currentColor" />
            <circle cx="11.5" cy="11.5" r=".5" fill="currentColor" />
            <circle cx="7.5" cy="16.5" r=".5" fill="currentColor" />
            <circle cx="17.5" cy="14.5" r=".5" fill="currentColor" />
            <path d="M3 3v16a2 2 0 0 0 2 2h16" />
        </svg>
    }
}
