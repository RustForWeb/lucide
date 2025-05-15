use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct SprayCanProps {
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
pub fn SprayCan(props: SprayCanProps) -> Element {
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
            path { "d": "M3 3h.01" }
            path { "d": "M7 5h.01" }
            path { "d": "M11 7h.01" }
            path { "d": "M3 7h.01" }
            path { "d": "M7 9h.01" }
            path { "d": "M3 11h.01" }
            rect {
                "width": "4",
                "height": "4",
                "x": "15",
                "y": "5",
            }
            path { "d": "m19 9 2 2v10c0 .6-.4 1-1 1h-6c-.6 0-1-.4-1-1V11l2-2" }
            path { "d": "m13 14 8-2" }
            path { "d": "m13 19 8-2" }
        }
    }
}
