use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct CircleDashedProps {
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
pub fn CircleDashed(props: CircleDashedProps) -> Element {
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
            path { "d": "M10.1 2.182a10 10 0 0 1 3.8 0" }
            path { "d": "M13.9 21.818a10 10 0 0 1-3.8 0" }
            path { "d": "M17.609 3.721a10 10 0 0 1 2.69 2.7" }
            path { "d": "M2.182 13.9a10 10 0 0 1 0-3.8" }
            path { "d": "M20.279 17.609a10 10 0 0 1-2.7 2.69" }
            path { "d": "M21.818 10.1a10 10 0 0 1 0 3.8" }
            path { "d": "M3.721 6.391a10 10 0 0 1 2.7-2.69" }
            path { "d": "M6.391 20.279a10 10 0 0 1-2.69-2.7" }
        }
    }
}
