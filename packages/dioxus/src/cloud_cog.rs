use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct CloudCogProps {
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
pub fn CloudCog(props: CloudCogProps) -> Element {
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
            circle { "cx": "12", "cy": "17", "r": "3" }
            path { "d": "M4.2 15.1A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.2" }
            path { "d": "m15.7 18.4-.9-.3" }
            path { "d": "m9.2 15.9-.9-.3" }
            path { "d": "m10.6 20.7.3-.9" }
            path { "d": "m13.1 14.2.3-.9" }
            path { "d": "m13.6 20.7-.4-1" }
            path { "d": "m10.8 14.3-.4-1" }
            path { "d": "m8.3 18.6 1-.4" }
            path { "d": "m14.7 15.8 1-.4" }
        }
    }
}
