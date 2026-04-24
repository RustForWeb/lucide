use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct RadioOffProps {
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
pub fn RadioOff(props: &RadioOffProps) -> Html {
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
            <path
                d="M10.4103 10.7852C10.1529 11.1218 10 11.5425 10 11.999C10 13.1036 10.8954 13.999 12 13.999C12.5077 13.999 12.9713 13.8098 13.324 13.498"
            />
            <path d="M16.1992 7.80078C17.4739 9.07549 18.0422 10.8109 17.9039 12.5134" />
            <path d="M19.0996 4.89844C22.0892 7.88804 22.7871 12.2879 21.1932 15.936" />
            <path d="M2 2L22 22" />
            <path d="M4.89961 19.0984C0.999609 15.1984 0.999609 8.79844 4.89961 4.89844" />
            <path d="M7.79922 16.1992C5.66828 14.0683 5.51165 10.6498 7.32931 8.25" />
        </svg>
    }
}
