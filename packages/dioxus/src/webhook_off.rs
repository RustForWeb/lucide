use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct WebhookOffProps {
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
pub fn WebhookOff(props: WebhookOffProps) -> Element {
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
            path { "d": "M17 17h-5c-1.09-.02-1.94.92-2.5 1.9A3 3 0 1 1 2.57 15" }
            path { "d": "M9 3.4a4 4 0 0 1 6.52.66" }
            path { "d": "m6 17 3.1-5.8a2.5 2.5 0 0 0 .057-2.05" }
            path { "d": "M20.3 20.3a4 4 0 0 1-2.3.7" }
            path { "d": "M18.6 13a4 4 0 0 1 3.357 3.414" }
            path { "d": "m12 6 .6 1" }
            path { "d": "m2 2 20 20" }
        }
    }
}
