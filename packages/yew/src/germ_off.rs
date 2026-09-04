use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct GermOffProps {
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
pub fn GermOff(props: &GermOffProps) -> Html {
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
            <path d="m11 2 .925 1.848" />
            <path d="M13 15h.01" />
            <path d="M13.424 7.768a2 2 0 112.808 2.808" />
            <path d="m16 21-1-2.472" />
            <path d="M16.988 16.988A12 12 0 019 20a5 5 0 01-2.759-9.171 8.8 8.8 0 002.307-2.28" />
            <path d="m19 2-1 1.804" />
            <path d="m2 19 2.746-1.373" />
            <path d="m2 2 20 20" />
            <path d="m22 16-2.474-2.13a12 12 0 001.376-3.786 6 6 0 00-10.313-5.151" />
            <path d="m22 5-1.804 1" />
            <path d="m3 10 2 2" />
            <path d="M9 16h.01" />
            <path d="M9 20v2" />
        </svg>
    }
}
