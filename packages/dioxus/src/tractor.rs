use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct TractorProps {
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
pub fn Tractor(props: TractorProps) -> Element {
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
            path { "d": "m10 11 11 .9a1 1 0 0 1 .8 1.1l-.665 4.158a1 1 0 0 1-.988.842H20" }
            path { "d": "M16 18h-5" }
            path { "d": "M18 5a1 1 0 0 0-1 1v5.573" }
            path { "d": "M3 4h8.129a1 1 0 0 1 .99.863L13 11.246" }
            path { "d": "M4 11V4" }
            path { "d": "M7 15h.01" }
            path { "d": "M8 10.1V4" }
            circle { "cx": "18", "cy": "18", "r": "2" }
            circle { "cx": "7", "cy": "15", "r": "5" }
        }
    }
}
