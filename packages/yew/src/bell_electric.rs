use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct BellElectricProps {
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
pub fn BellElectric(props: &BellElectricProps) -> Html {
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
            <path d="M18.8 4A6.3 8.7 0 0 1 20 9" />
            <path d="M9 9h.01" />
            <circle cx="9" cy="9" r="7" />
            <rect width="10" height="6" x="4" y="16" rx="2" />
            <path d="M14 19c3 0 4.6-1.6 4.6-1.6" />
            <circle cx="20" cy="16" r="2" />
        </svg>
    }
}