use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct SquircleDashedProps {
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
pub fn SquircleDashed(props: &SquircleDashedProps) -> Html {
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
            <path d="M13.77 3.043a34 34 0 0 0-3.54 0" />
            <path d="M13.771 20.956a33 33 0 0 1-3.541.001" />
            <path d="M20.18 17.74c-.51 1.15-1.29 1.93-2.439 2.44" />
            <path d="M20.18 6.259c-.51-1.148-1.291-1.929-2.44-2.438" />
            <path d="M20.957 10.23a33 33 0 0 1 0 3.54" />
            <path d="M3.043 10.23a34 34 0 0 0 .001 3.541" />
            <path d="M6.26 20.179c-1.15-.508-1.93-1.29-2.44-2.438" />
            <path d="M6.26 3.82c-1.149.51-1.93 1.291-2.44 2.44" />
        </svg>
    }
}
