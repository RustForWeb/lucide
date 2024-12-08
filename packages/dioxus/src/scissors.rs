use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct ScissorsProps {
    #[props(default = 24)]
    pub size: usize,
    #[props(default = "currentColor".to_owned())]
    pub color: String,
    #[props(default = "none".to_owned())]
    pub fill: String,
    #[props(default = 2)]
    pub stroke_width: usize,
    #[props(default = false)]
    pub absolute_stroke_width: bool,
    pub class: Option<String>,
}
#[component]
pub fn Scissors(props: ScissorsProps) -> Element {
    let stroke_width = if props.absolute_stroke_width {
        props.stroke_width * 24 / props.size
    } else {
        props.stroke_width
    };
    rsx! {
        svg {
            "xmlns": "http://www.w3.org/2000/svg",
            "class": if let Some(class) = props.class { "{class}" },
            "width": "{props.size}",
            "height": "{props.size}",
            "viewBox": "0 0 24 24",
            "fill": "{props.fill}",
            "stroke": "{props.color}",
            "stroke-width": "{stroke_width}",
            "stroke-linecap": "round",
            "stroke-linejoin": "round",
            circle { "cx": "6", "cy": "6", "r": "3" }
            path { "d": "M8.12 8.12 12 12" }
            path { "d": "M20 4 8.12 15.88" }
            circle { "cx": "6", "cy": "18", "r": "3" }
            path { "d": "M14.8 14.8 20 20" }
        }
    }
}
