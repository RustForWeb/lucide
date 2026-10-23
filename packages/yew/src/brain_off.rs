use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct BrainOffProps {
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
pub fn BrainOff(props: &BrainOffProps) -> Html {
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
            <path d="M12 18v-6" />
            <path d="M12 5v1.344" />
            <path d="M17.707 6.293A3 3 0 1012 5a3 3 0 00-4.124-2.781" />
            <path d="M17.997 5.125a4 4 0 012.526 5.77" />
            <path d="M19.644 19.644A4 4 0 0112 18a4 4 0 11-7.967-.517" />
            <path d="m2 2 20 20" />
            <path d="M20 10.536a4 4 0 011.533 5.34" />
            <path d="M3.477 10.895a4 4 0 011.876-5.542" />
            <path d="M6 18a4 4 0 01-2-7.464" />
            <path d="M9 13a4.17 4.17 0 002.166-1.5 4 4 0 00.138-.196" />
        </svg>
    }
}
