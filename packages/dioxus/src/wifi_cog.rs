use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct WifiCogProps {
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
pub fn WifiCog(props: WifiCogProps) -> Element {
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
            path { "d": "m14.305 19.53.923-.382" }
            path { "d": "m15.228 16.852-.923-.383" }
            path { "d": "m16.852 15.228-.383-.923" }
            path { "d": "m16.852 20.772-.383.924" }
            path { "d": "m19.148 15.228.383-.923" }
            path { "d": "m19.53 21.696-.382-.924" }
            path { "d": "M2 7.82a15 15 0 0 1 20 0" }
            path { "d": "m20.772 16.852.924-.383" }
            path { "d": "m20.772 19.148.924.383" }
            path { "d": "M5 11.858a10 10 0 0 1 11.5-1.785" }
            path { "d": "M8.5 15.429a5 5 0 0 1 2.413-1.31" }
            circle { "cx": "18", "cy": "18", "r": "3" }
        }
    }
}
