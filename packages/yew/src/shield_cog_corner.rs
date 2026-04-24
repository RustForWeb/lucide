use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct ShieldCogCornerProps {
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
pub fn ShieldCogCorner(props: &ShieldCogCornerProps) -> Html {
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
                d="M11 22c-3.806-1.45-7-3.966-7-9V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1v4"
            />
            <path d="M14.923 16.547 14 16.164" />
            <path d="m14.923 18.843-.923.383" />
            <path d="M16.547 14.923 16.164 14" />
            <path d="m16.547 20.467-.383.924" />
            <path d="m18.843 14.923.383-.923" />
            <path d="m19.225 21.391-.382-.924" />
            <path d="m20.467 16.547.923-.383" />
            <path d="m20.467 18.843.923.383" />
            <circle cx="17.695" cy="17.695" r="3" />
        </svg>
    }
}
