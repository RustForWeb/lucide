use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct CombineProps {
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
pub fn Combine(props: CombineProps) -> Element {
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
            path { "d": "M10 18H5a3 3 0 0 1-3-3v-1" }
            path { "d": "M14 2a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2" }
            path { "d": "M20 2a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2" }
            path { "d": "m7 21 3-3-3-3" }
            rect {
                "x": "14",
                "y": "14",
                "width": "8",
                "height": "8",
                "rx": "2",
            }
            rect {
                "x": "2",
                "y": "2",
                "width": "8",
                "height": "8",
                "rx": "2",
            }
        }
    }
}
