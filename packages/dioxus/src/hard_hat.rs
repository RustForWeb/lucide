use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct HardHatProps {
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
pub fn HardHat(props: HardHatProps) -> Element {
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
            path { "d": "M10 10V5a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1v5" }
            path { "d": "M14 6a6 6 0 0 1 6 6v3" }
            path { "d": "M4 15v-3a6 6 0 0 1 6-6" }
            rect {
                "x": "2",
                "y": "15",
                "width": "20",
                "height": "4",
                "rx": "1",
            }
        }
    }
}
