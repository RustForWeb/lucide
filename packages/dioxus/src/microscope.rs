use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct MicroscopeProps {
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
pub fn Microscope(props: MicroscopeProps) -> Element {
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
            path { "d": "M6 18h8" }
            path { "d": "M3 22h18" }
            path { "d": "M14 22a7 7 0 1 0 0-14h-1" }
            path { "d": "M9 14h2" }
            path { "d": "M9 12a2 2 0 0 1-2-2V6h6v4a2 2 0 0 1-2 2Z" }
            path { "d": "M12 6V3a1 1 0 0 0-1-1H9a1 1 0 0 0-1 1v3" }
        }
    }
}