use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct ChromeProps {
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
pub fn Chrome(props: ChromeProps) -> Element {
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
            circle { "cx": "12", "cy": "12", "r": "10" }
            circle { "cx": "12", "cy": "12", "r": "4" }
            line {
                "x1": "21.17",
                "x2": "12",
                "y1": "8",
                "y2": "8",
            }
            line {
                "x1": "3.95",
                "x2": "8.54",
                "y1": "6.06",
                "y2": "14",
            }
            line {
                "x1": "10.88",
                "x2": "15.46",
                "y1": "21.94",
                "y2": "14",
            }
        }
    }
}
