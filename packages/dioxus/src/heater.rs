use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct HeaterProps {
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
pub fn Heater(props: HeaterProps) -> Element {
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
            path { "d": "M11 8c2-3-2-3 0-6" }
            path { "d": "M15.5 8c2-3-2-3 0-6" }
            path { "d": "M6 10h.01" }
            path { "d": "M6 14h.01" }
            path { "d": "M10 16v-4" }
            path { "d": "M14 16v-4" }
            path { "d": "M18 16v-4" }
            path { "d": "M20 6a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h3" }
            path { "d": "M5 20v2" }
            path { "d": "M19 20v2" }
        }
    }
}
