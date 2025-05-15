use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct GlassesProps {
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
pub fn Glasses(props: GlassesProps) -> Element {
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
            circle { "cx": "6", "cy": "15", "r": "4" }
            circle { "cx": "18", "cy": "15", "r": "4" }
            path { "d": "M14 15a2 2 0 0 0-2-2 2 2 0 0 0-2 2" }
            path { "d": "M2.5 13 5 7c.7-1.3 1.4-2 3-2" }
            path { "d": "M21.5 13 19 7c-.7-1.3-1.5-2-3-2" }
        }
    }
}
