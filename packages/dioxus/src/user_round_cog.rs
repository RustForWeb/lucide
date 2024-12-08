use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct UserRoundCogProps {
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
}
#[component]
pub fn UserRoundCog(props: UserRoundCogProps) -> Element {
    let stroke_width = if props.absolute_stroke_width {
        props.stroke_width * 24 / props.size
    } else {
        props.stroke_width
    };
    rsx! {
        svg {
            "xmlns": "http://www.w3.org/2000/svg",
            "class": if let Some(class) = props.class { "{class}" },
            "width": "{props.size}",
            "height": "{props.size}",
            "viewBox": "0 0 24 24",
            "fill": "{props.fill}",
            "stroke": "{props.color}",
            "stroke-width": "{stroke_width}",
            "stroke-linecap": "round",
            "stroke-linejoin": "round",
            path { "d": "M2 21a8 8 0 0 1 10.434-7.62" }
            circle { "cx": "10", "cy": "8", "r": "5" }
            circle { "cx": "18", "cy": "18", "r": "3" }
            path { "d": "m19.5 14.3-.4.9" }
            path { "d": "m16.9 20.8-.4.9" }
            path { "d": "m21.7 19.5-.9-.4" }
            path { "d": "m15.2 16.9-.9-.4" }
            path { "d": "m21.7 16.5-.9.4" }
            path { "d": "m15.2 19.1-.9.4" }
            path { "d": "m19.5 21.7-.4-.9" }
            path { "d": "m16.9 15.2-.4-.9" }
        }
    }
}