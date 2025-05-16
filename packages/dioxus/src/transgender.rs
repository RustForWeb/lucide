use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct TransgenderProps {
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
    pub style: Option<String>,
}
#[component]
pub fn Transgender(props: TransgenderProps) -> Element {
    let stroke_width = if props.absolute_stroke_width {
        props.stroke_width * 24 / props.size
    } else {
        props.stroke_width
    };
    rsx! {
        svg {
            "xmlns": "http://www.w3.org/2000/svg",
            "class": if let Some(class) = props.class { "{class}" },
            "style": if let Some(style) = props.style { "{style}" },
            "width": "{props.size}",
            "height": "{props.size}",
            "viewBox": "0 0 24 24",
            "fill": "{props.fill}",
            "stroke": "{props.color}",
            "stroke-width": "{stroke_width}",
            "stroke-linecap": "round",
            "stroke-linejoin": "round",
            path { "d": "M12 16v6" }
            path { "d": "M14 20h-4" }
            path { "d": "M18 2h4v4" }
            path { "d": "m2 2 7.17 7.17" }
            path { "d": "M2 5.355V2h3.357" }
            path { "d": "m22 2-7.17 7.17" }
            path { "d": "M8 5 5 8" }
            circle { "cx": "12", "cy": "12", "r": "4" }
        }
    }
}
