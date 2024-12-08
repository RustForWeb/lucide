use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct SheetProps {
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
pub fn Sheet(props: SheetProps) -> Element {
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
            rect {
                "width": "18",
                "height": "18",
                "x": "3",
                "y": "3",
                "rx": "2",
                "ry": "2",
            }
            line {
                "x1": "3",
                "x2": "21",
                "y1": "9",
                "y2": "9",
            }
            line {
                "x1": "3",
                "x2": "21",
                "y1": "15",
                "y2": "15",
            }
            line {
                "x1": "9",
                "x2": "9",
                "y1": "9",
                "y2": "21",
            }
            line {
                "x1": "15",
                "x2": "15",
                "y1": "9",
                "y2": "21",
            }
        }
    }
}