use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct UsersRoundCollaborationProps {
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
pub fn UsersRoundCollaboration(props: &UsersRoundCollaborationProps) -> Html {
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
            <path d="M20.453 8.911a9 9 0 00-6.696-5.738" />
            <path d="M21 22a4 4 0 00-8 0" />
            <path d="M3 12a4 4 0 017.963-.543" />
            <path d="M3.889 15.9a9 9 0 005.247 4.632" />
            <circle cx="17" cy="15" r="3" />
            <circle cx="7" cy="5" r="3" />
        </svg>
    }
}
