use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct BiohazardProps {
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
pub fn Biohazard(props: BiohazardProps) -> Element {
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
            circle { "cx": "12", "cy": "11.9", "r": "2" }
            path { "d": "M6.7 3.4c-.9 2.5 0 5.2 2.2 6.7C6.5 9 3.7 9.6 2 11.6" }
            path { "d": "m8.9 10.1 1.4.8" }
            path { "d": "M17.3 3.4c.9 2.5 0 5.2-2.2 6.7 2.4-1.2 5.2-.6 6.9 1.5" }
            path { "d": "m15.1 10.1-1.4.8" }
            path { "d": "M16.7 20.8c-2.6-.4-4.6-2.6-4.7-5.3-.2 2.6-2.1 4.8-4.7 5.2" }
            path { "d": "M12 13.9v1.6" }
            path { "d": "M13.5 5.4c-1-.2-2-.2-3 0" }
            path { "d": "M17 16.4c.7-.7 1.2-1.6 1.5-2.5" }
            path { "d": "M5.5 13.9c.3.9.8 1.8 1.5 2.5" }
        }
    }
}
