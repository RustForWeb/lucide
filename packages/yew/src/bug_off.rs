use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct BugOffProps {
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
pub fn BugOff(props: &BugOffProps) -> Html {
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
            <path d="M12 20v-8" />
            <path d="M12.656 7H14a4 4 0 0 1 4 4v1.344" />
            <path d="M14.12 3.88 16 2" />
            <path d="M17.123 17.123A6 6 0 0 1 6 14v-3a4 4 0 0 1 1.72-3.287" />
            <path d="m2 2 20 20" />
            <path d="M21 5a4 4 0 0 1-3.55 3.97" />
            <path d="M22 13h-3.344" />
            <path d="M3 21a4 4 0 0 1 3.81-4" />
            <path d="M3 5a4 4 0 0 0 3.55 3.97" />
            <path d="M6 13H2" />
            <path d="m8 2 1.88 1.88" />
            <path d="M9.712 4.06A3 3 0 0 1 15 6v1.13" />
        </svg>
    }
}
