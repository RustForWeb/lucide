use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct TheaterProps {
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
pub fn Theater(props: TheaterProps) -> Element {
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
            path { "d": "M2 10s3-3 3-8" }
            path { "d": "M22 10s-3-3-3-8" }
            path { "d": "M10 2c0 4.4-3.6 8-8 8" }
            path { "d": "M14 2c0 4.4 3.6 8 8 8" }
            path { "d": "M2 10s2 2 2 5" }
            path { "d": "M22 10s-2 2-2 5" }
            path { "d": "M8 15h8" }
            path { "d": "M2 22v-1a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v1" }
            path { "d": "M14 22v-1a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v1" }
        }
    }
}
