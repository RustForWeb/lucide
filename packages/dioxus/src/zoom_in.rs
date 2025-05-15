use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct ZoomInProps {
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
pub fn ZoomIn(props: ZoomInProps) -> Element {
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
            circle { "cx": "11", "cy": "11", "r": "8" }
            line {
                "x1": "21",
                "x2": "16.65",
                "y1": "21",
                "y2": "16.65",
            }
            line {
                "x1": "11",
                "x2": "11",
                "y1": "8",
                "y2": "14",
            }
            line {
                "x1": "8",
                "x2": "14",
                "y1": "11",
                "y2": "11",
            }
        }
    }
}
