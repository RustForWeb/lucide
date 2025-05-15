use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct CodepenProps {
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
pub fn Codepen(props: CodepenProps) -> Element {
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
            polygon { "points": "12 2 22 8.5 22 15.5 12 22 2 15.5 2 8.5 12 2" }
            line {
                "x1": "12",
                "x2": "12",
                "y1": "22",
                "y2": "15.5",
            }
            polyline { "points": "22 8.5 12 15.5 2 8.5" }
            polyline { "points": "2 15.5 12 8.5 22 15.5" }
            line {
                "x1": "12",
                "x2": "12",
                "y1": "2",
                "y2": "8.5",
            }
        }
    }
}
