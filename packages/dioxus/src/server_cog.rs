use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct ServerCogProps {
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
pub fn ServerCog(props: ServerCogProps) -> Element {
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
            circle { "cx": "12", "cy": "12", "r": "3" }
            path { "d": "M4.5 10H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2h-.5" }
            path { "d": "M4.5 14H4a2 2 0 0 0-2 2v4a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-4a2 2 0 0 0-2-2h-.5" }
            path { "d": "M6 6h.01" }
            path { "d": "M6 18h.01" }
            path { "d": "m15.7 13.4-.9-.3" }
            path { "d": "m9.2 10.9-.9-.3" }
            path { "d": "m10.6 15.7.3-.9" }
            path { "d": "m13.6 15.7-.4-1" }
            path { "d": "m10.8 9.3-.4-1" }
            path { "d": "m8.3 13.6 1-.4" }
            path { "d": "m14.7 10.8 1-.4" }
            path { "d": "m13.4 8.3-.3.9" }
        }
    }
}
