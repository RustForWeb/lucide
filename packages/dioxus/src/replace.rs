use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct ReplaceProps {
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
pub fn Replace(props: ReplaceProps) -> Element {
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
            path { "d": "M14 4a1 1 0 0 1 1-1" }
            path { "d": "M15 10a1 1 0 0 1-1-1" }
            path { "d": "M21 4a1 1 0 0 0-1-1" }
            path { "d": "M21 9a1 1 0 0 1-1 1" }
            path { "d": "m3 7 3 3 3-3" }
            path { "d": "M6 10V5a2 2 0 0 1 2-2h2" }
            rect {
                "x": "3",
                "y": "14",
                "width": "7",
                "height": "7",
                "rx": "1",
            }
        }
    }
}
