use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct RouteOffProps {
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
pub fn RouteOff(props: RouteOffProps) -> Element {
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
            circle { "cx": "6", "cy": "19", "r": "3" }
            path { "d": "M9 19h8.5c.4 0 .9-.1 1.3-.2" }
            path { "d": "M5.2 5.2A3.5 3.53 0 0 0 6.5 12H12" }
            path { "d": "m2 2 20 20" }
            path { "d": "M21 15.3a3.5 3.5 0 0 0-3.3-3.3" }
            path { "d": "M15 5h-4.3" }
            circle { "cx": "18", "cy": "5", "r": "3" }
        }
    }
}
