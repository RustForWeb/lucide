use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct CameraOffProps {
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
pub fn CameraOff(props: CameraOffProps) -> Element {
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
            path { "d": "M7 7H4a2 2 0 0 0-2 2v9a2 2 0 0 0 2 2h16" }
            path { "d": "M9.5 4h5L17 7h3a2 2 0 0 1 2 2v7.5" }
            path { "d": "M14.121 15.121A3 3 0 1 1 9.88 10.88" }
        }
    }
}
