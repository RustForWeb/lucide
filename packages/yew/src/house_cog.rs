use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct HouseCogProps {
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
pub fn HouseCog(props: &HouseCogProps) -> Html {
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
                d="M10.584 21H5a2 2 0 01-2-2v-9a2 2 0 01.709-1.527l7-6a2 2 0 012.582 0l7 6A2 2 0 0121 10.001v.583"
            />
            <path d="M14 12H10a1 1 0 00-1 1v8" />
            <path d="m14.305 19.53.923-.382" />
            <path d="m15.229 16.852-.924-.383" />
            <path d="m16.852 15.228-.383-.923" />
            <path d="m16.852 20.773-.383.924" />
            <path d="m19.148 15.228.383-.923" />
            <path d="m19.53 21.697-.382-.924" />
            <path d="m20.773 16.852.922-.383" />
            <path d="m20.773 19.148.922.383" />
            <circle cx="18" cy="18" r="3" />
        </svg>
    }
}
