use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct Minimize2Props {
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
pub fn Minimize2(props: Minimize2Props) -> Element {
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
            polyline { "points": "4 14 10 14 10 20" }
            polyline { "points": "20 10 14 10 14 4" }
            line {
                "x1": "14",
                "x2": "21",
                "y1": "10",
                "y2": "3",
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
