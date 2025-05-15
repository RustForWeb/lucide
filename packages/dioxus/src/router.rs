use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct RouterProps {
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
pub fn Router(props: RouterProps) -> Element {
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
                "width": "20",
                "height": "8",
                "x": "2",
                "y": "14",
                "rx": "2",
            }
            path { "d": "M6.01 18H6" }
            path { "d": "M10.01 18H10" }
            path { "d": "M15 10v4" }
            path { "d": "M17.84 7.17a4 4 0 0 0-5.66 0" }
            path { "d": "M20.66 4.34a8 8 0 0 0-11.31 0" }
        }
    }
}
