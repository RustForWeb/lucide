use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct UsbProps {
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
pub fn Usb(props: UsbProps) -> Element {
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
            circle { "cx": "10", "cy": "7", "r": "1" }
            circle { "cx": "4", "cy": "20", "r": "1" }
            path { "d": "M4.7 19.3 19 5" }
            path { "d": "m21 3-3 1 2 2Z" }
            path { "d": "M9.26 7.68 5 12l2 5" }
            path { "d": "m10 14 5 2 3.5-3.5" }
            path { "d": "m18 12 1-1 1 1-1 1Z" }
        }
    }
}
