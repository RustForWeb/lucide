use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct TicketsPlaneProps {
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
pub fn TicketsPlane(props: TicketsPlaneProps) -> Element {
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
            path { "d": "M10.5 17h1.227a2 2 0 0 0 1.345-.52L18 12" }
            path { "d": "m12 13.5 3.75.5" }
            path { "d": "m4.5 8 10.58-5.06a1 1 0 0 1 1.342.488L18.5 8" }
            path { "d": "M6 10V8" }
            path { "d": "M6 14v1" }
            path { "d": "M6 19v2" }
            rect {
                "x": "2",
                "y": "8",
                "width": "20",
                "height": "13",
                "rx": "2",
            }
        }
    }
}
