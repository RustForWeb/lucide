use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct UserRoundCogProps {
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
pub fn UserRoundCog(props: &UserRoundCogProps) -> Html {
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
            <path d="M2 21a8 8 0 0 1 10.434-7.62" />
            <circle cx="10" cy="8" r="5" />
            <circle cx="18" cy="18" r="3" />
            <path d="m19.5 14.3-.4.9" />
            <path d="m16.9 20.8-.4.9" />
            <path d="m21.7 19.5-.9-.4" />
            <path d="m15.2 16.9-.9-.4" />
            <path d="m21.7 16.5-.9.4" />
            <path d="m15.2 19.1-.9.4" />
            <path d="m19.5 21.7-.4-.9" />
            <path d="m16.9 15.2-.4-.9" />
        </svg>
    }
}
