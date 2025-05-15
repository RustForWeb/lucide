use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct Maximize2Props {
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
pub fn Maximize2(props: Maximize2Props) -> Element {
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
            polyline { "points": "15 3 21 3 21 9" }
            polyline { "points": "9 21 3 21 3 15" }
            line {
                "x1": "21",
                "x2": "14",
                "y1": "3",
                "y2": "10",
            }
            line {
                "x1": "3",
                "x2": "10",
                "y1": "21",
                "y2": "14",
            }
        }
    }
}
