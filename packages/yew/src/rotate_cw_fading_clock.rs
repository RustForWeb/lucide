use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct RotateCwFadingClockProps {
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
pub fn RotateCwFadingClock(props: &RotateCwFadingClockProps) -> Html {
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
            <path d="M12 3a9.75 9.75 0 0 1 6.74 2.74" />
            <path d="M18.74 5.74 21 8" />
            <path d="M21 8V3" />
            <path d="M7.5 19.794c-6-3.464-6-12.124 0-15.588" />
            <path d="M7.5 4.206A9 9 0 0 1 12 3" />
            <path d="M12 7v5l4 2" />
            <path d="M14 20.775A9 9 0 0 1 12 21" />
            <path d="M19 17.656a9 9 0 0 1-1.5 1.456" />
            <path d="M21 12a9 9 0 0 1-.228 2" />
            <path d="M21 8h-5" />
        </svg>
    }
}
