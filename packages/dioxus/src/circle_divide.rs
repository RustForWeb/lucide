use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct CircleDivideProps {
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
pub fn CircleDivide(props: CircleDivideProps) -> Element {
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
            line {
                "x1": "8",
                "x2": "16",
                "y1": "12",
                "y2": "12",
            }
            line {
                "x1": "12",
                "x2": "12",
                "y1": "16",
                "y2": "16",
            }
            line {
                "x1": "12",
                "x2": "12",
                "y1": "8",
                "y2": "8",
            }
            circle { "cx": "12", "cy": "12", "r": "10" }
        }
    }
}