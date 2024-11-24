use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct GitPullRequestCreateArrowProps {
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
pub fn GitPullRequestCreateArrow(props: &GitPullRequestCreateArrowProps) -> Html {
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
            <circle cx="5" cy="6" r="3" />
            <path d="M5 9v12" />
            <path d="m15 9-3-3 3-3" />
            <path d="M12 6h5a2 2 0 0 1 2 2v3" />
            <path d="M19 15v6" />
            <path d="M22 18h-6" />
        </svg>
    }
}