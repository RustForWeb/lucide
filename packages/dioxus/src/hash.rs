use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct HashProps {
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
pub fn Hash(props: HashProps) -> Element {
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
            line {
                "x1": "4",
                "x2": "20",
                "y1": "9",
                "y2": "9",
            }
            line {
                "x1": "4",
                "x2": "20",
                "y1": "15",
                "y2": "15",
            }
            line {
                "x1": "10",
                "x2": "8",
                "y1": "3",
                "y2": "21",
            }
            line {
                "x1": "16",
                "x2": "14",
                "y1": "3",
                "y2": "21",
            }
        }
    }
}
