use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct VectorSquareProps {
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
pub fn VectorSquare(props: VectorSquareProps) -> Element {
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
            path { "d": "M19.5 7a24 24 0 0 1 0 10" }
            path { "d": "M4.5 7a24 24 0 0 0 0 10" }
            path { "d": "M7 19.5a24 24 0 0 0 10 0" }
            path { "d": "M7 4.5a24 24 0 0 1 10 0" }
            rect {
                "x": "17",
                "y": "17",
                "width": "5",
                "height": "5",
                "rx": "1",
            }
            rect {
                "x": "17",
                "y": "2",
                "width": "5",
                "height": "5",
                "rx": "1",
            }
            rect {
                "x": "2",
                "y": "17",
                "width": "5",
                "height": "5",
                "rx": "1",
            }
            rect {
                "x": "2",
                "y": "2",
                "width": "5",
                "height": "5",
                "rx": "1",
            }
        }
    }
}
