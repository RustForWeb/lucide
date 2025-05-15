use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct ShowerHeadProps {
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
pub fn ShowerHead(props: ShowerHeadProps) -> Element {
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
            path { "d": "m4 4 2.5 2.5" }
            path { "d": "M13.5 6.5a4.95 4.95 0 0 0-7 7" }
            path { "d": "M15 5 5 15" }
            path { "d": "M14 17v.01" }
            path { "d": "M10 16v.01" }
            path { "d": "M13 13v.01" }
            path { "d": "M16 10v.01" }
            path { "d": "M11 20v.01" }
            path { "d": "M17 14v.01" }
            path { "d": "M20 11v.01" }
        }
    }
}
