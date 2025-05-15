use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct DatabaseZapProps {
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
pub fn DatabaseZap(props: DatabaseZapProps) -> Element {
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
            ellipse {
                "cx": "12",
                "cy": "5",
                "rx": "9",
                "ry": "3",
            }
            path { "d": "M3 5V19A9 3 0 0 0 15 21.84" }
            path { "d": "M21 5V8" }
            path { "d": "M21 12L18 17H22L19 22" }
            path { "d": "M3 12A9 3 0 0 0 14.59 14.87" }
        }
    }
}
