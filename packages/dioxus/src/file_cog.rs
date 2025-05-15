use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct FileCogProps {
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
pub fn FileCog(props: FileCogProps) -> Element {
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
            path { "d": "M14 2v4a2 2 0 0 0 2 2h4" }
            path { "d": "m3.2 12.9-.9-.4" }
            path { "d": "m3.2 15.1-.9.4" }
            path { "d": "M4.677 21.5a2 2 0 0 0 1.313.5H18a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v2.5" }
            path { "d": "m4.9 11.2-.4-.9" }
            path { "d": "m4.9 16.8-.4.9" }
            path { "d": "m7.5 10.3-.4.9" }
            path { "d": "m7.5 17.7-.4-.9" }
            path { "d": "m9.7 12.5-.9.4" }
            path { "d": "m9.7 15.5-.9-.4" }
            circle { "cx": "6", "cy": "14", "r": "3" }
        }
    }
}
