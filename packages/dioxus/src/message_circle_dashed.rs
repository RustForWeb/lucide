use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct MessageCircleDashedProps {
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
pub fn MessageCircleDashed(props: MessageCircleDashedProps) -> Element {
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
            path { "d": "M13.5 3.1c-.5 0-1-.1-1.5-.1s-1 .1-1.5.1" }
            path { "d": "M19.3 6.8a10.45 10.45 0 0 0-2.1-2.1" }
            path { "d": "M20.9 13.5c.1-.5.1-1 .1-1.5s-.1-1-.1-1.5" }
            path { "d": "M17.2 19.3a10.45 10.45 0 0 0 2.1-2.1" }
            path { "d": "M10.5 20.9c.5.1 1 .1 1.5.1s1-.1 1.5-.1" }
            path { "d": "M3.5 17.5 2 22l4.5-1.5" }
            path { "d": "M3.1 10.5c0 .5-.1 1-.1 1.5s.1 1 .1 1.5" }
            path { "d": "M6.8 4.7a10.45 10.45 0 0 0-2.1 2.1" }
        }
    }
}
