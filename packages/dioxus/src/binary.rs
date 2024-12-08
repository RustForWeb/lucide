use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct BinaryProps {
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
pub fn Binary(props: BinaryProps) -> Element {
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
                "x": "14",
                "y": "14",
                "width": "4",
                "height": "6",
                "rx": "2",
            }
            rect {
                "x": "6",
                "y": "4",
                "width": "4",
                "height": "6",
                "rx": "2",
            }
            path { "d": "M6 20h4" }
            path { "d": "M14 10h4" }
            path { "d": "M6 14h2v6" }
            path { "d": "M14 4h2v6" }
        }
    }
}
