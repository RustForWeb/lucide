use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct BrainCogProps {
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
pub fn BrainCog(props: &BrainCogProps) -> Html {
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
            <path d="m10.852 14.772-.383.923" />
            <path d="m10.852 9.228-.383-.923" />
            <path d="m13.148 14.772.382.924" />
            <path d="m13.53 8.305-.382.923" />
            <path d="m14.772 10.852.923-.383" />
            <path d="m14.772 13.148.923.383" />
            <path d="M17.997 5.125a4 4 0 012.526 5.77" />
            <path d="M18 18a4 4 0 002-7.464" />
            <path d="M4.033 17.483A4 4 0 0011.46 20c.184-.31.896-.31 1.076 0a4 4 0 007.43-2.517" />
            <path d="M6 18a4 4 0 01-2-7.464" />
            <path d="M6.002 5.125a4 4 0 00-2.525 5.77" />
            <path
                d="M6.293 6.294a3 3 0 01-.287-1.098l-.004-.07a3 3 0 015.968-.543c.05.02.057.01.059 0a3 3 0 015.968.542l-.004.071a3 3 0 01-.287 1.098"
            />
            <path d="m9.228 10.852-.923-.383" />
            <path d="m9.228 13.148-.923.383" />
            <circle cx="12" cy="12" r="3" />
        </svg>
    }
}
