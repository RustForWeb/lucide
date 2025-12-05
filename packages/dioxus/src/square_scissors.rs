use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct SquareScissorsProps {
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
pub fn SquareScissors(props: SquareScissorsProps) -> Element {
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
            rect {
                "width": "18",
                "height": "18",
                "x": "3",
                "y": "3",
                "rx": "2",
            }
            circle { "cx": "8.5", "cy": "8.5", "r": "1.5" }
            line {
                "x1": "9.56066",
                "y1": "9.56066",
                "x2": "12",
                "y2": "12",
            }
            line {
                "x1": "17",
                "y1": "17",
                "x2": "14.82",
                "y2": "14.82",
            }
            circle { "cx": "8.5", "cy": "15.5", "r": "1.5" }
            line {
                "x1": "9.56066",
                "y1": "14.43934",
                "x2": "17",
                "y2": "7",
            }
        }
    }
}
