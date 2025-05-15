use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct UserCogProps {
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
pub fn UserCog(props: UserCogProps) -> Element {
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
            circle { "cx": "18", "cy": "15", "r": "3" }
            circle { "cx": "9", "cy": "7", "r": "4" }
            path { "d": "M10 15H6a4 4 0 0 0-4 4v2" }
            path { "d": "m21.7 16.4-.9-.3" }
            path { "d": "m15.2 13.9-.9-.3" }
            path { "d": "m16.6 18.7.3-.9" }
            path { "d": "m19.1 12.2.3-.9" }
            path { "d": "m19.6 18.7-.4-1" }
            path { "d": "m16.8 12.3-.4-1" }
            path { "d": "m14.3 16.6 1-.4" }
            path { "d": "m20.7 13.8 1-.4" }
        }
    }
}
