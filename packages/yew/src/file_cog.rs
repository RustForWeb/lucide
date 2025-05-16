use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct FileCogProps {
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
pub fn FileCog(props: &FileCogProps) -> Html {
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
            <path d="M14 2v4a2 2 0 0 0 2 2h4" />
            <path d="m2.305 15.53.923-.382" />
            <path d="m3.228 12.852-.924-.383" />
            <path d="M4.677 21.5a2 2 0 0 0 1.313.5H18a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v2.5" />
            <path d="m4.852 11.228-.383-.923" />
            <path d="m4.852 16.772-.383.924" />
            <path d="m7.148 11.228.383-.923" />
            <path d="m7.53 17.696-.382-.924" />
            <path d="m8.772 12.852.923-.383" />
            <path d="m8.772 15.148.923.383" />
            <circle cx="6" cy="14" r="3" />
        </svg>
    }
}
