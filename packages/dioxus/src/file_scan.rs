use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct FileScanProps {
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
pub fn FileScan(props: FileScanProps) -> Element {
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
            path { "d": "M20 10V7l-5-5H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h4" }
            path { "d": "M14 2v4a2 2 0 0 0 2 2h4" }
            path { "d": "M16 14a2 2 0 0 0-2 2" }
            path { "d": "M20 14a2 2 0 0 1 2 2" }
            path { "d": "M20 22a2 2 0 0 0 2-2" }
            path { "d": "M16 22a2 2 0 0 1-2-2" }
        }
    }
}
