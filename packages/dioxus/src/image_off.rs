use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct ImageOffProps {
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
pub fn ImageOff(props: ImageOffProps) -> Element {
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
                "x1": "2",
                "x2": "22",
                "y1": "2",
                "y2": "22",
            }
            path { "d": "M10.41 10.41a2 2 0 1 1-2.83-2.83" }
            line {
                "x1": "13.5",
                "x2": "6",
                "y1": "13.5",
                "y2": "21",
            }
            line {
                "x1": "18",
                "x2": "21",
                "y1": "12",
                "y2": "15",
            }
            path { "d": "M3.59 3.59A1.99 1.99 0 0 0 3 5v14a2 2 0 0 0 2 2h14c.55 0 1.052-.22 1.41-.59" }
            path { "d": "M21 15V5a2 2 0 0 0-2-2H9" }
        }
    }
}
