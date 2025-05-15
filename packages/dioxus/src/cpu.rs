use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct CpuProps {
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
pub fn Cpu(props: CpuProps) -> Element {
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
            rect {
                "width": "16",
                "height": "16",
                "x": "4",
                "y": "4",
                "rx": "2",
            }
            rect {
                "width": "6",
                "height": "6",
                "x": "9",
                "y": "9",
                "rx": "1",
            }
            path { "d": "M15 2v2" }
            path { "d": "M15 20v2" }
            path { "d": "M2 15h2" }
            path { "d": "M2 9h2" }
            path { "d": "M20 15h2" }
            path { "d": "M20 9h2" }
            path { "d": "M9 2v2" }
            path { "d": "M9 20v2" }
        }
    }
}
