use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct KeyboardOffProps {
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
pub fn KeyboardOff(props: KeyboardOffProps) -> Element {
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
            path { "d": "M 20 4 A2 2 0 0 1 22 6" }
            path { "d": "M 22 6 L 22 16.41" }
            path { "d": "M 7 16 L 16 16" }
            path { "d": "M 9.69 4 L 20 4" }
            path { "d": "M14 8h.01" }
            path { "d": "M18 8h.01" }
            path { "d": "m2 2 20 20" }
            path { "d": "M20 20H4a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2" }
            path { "d": "M6 8h.01" }
            path { "d": "M8 12h.01" }
        }
    }
}