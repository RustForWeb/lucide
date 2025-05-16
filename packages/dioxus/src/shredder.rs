use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct ShredderProps {
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
pub fn Shredder(props: ShredderProps) -> Element {
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
            path { "d": "M10 22v-5" }
            path { "d": "M14 19v-2" }
            path { "d": "M14 2v4a2 2 0 0 0 2 2h4" }
            path { "d": "M18 20v-3" }
            path { "d": "M2 13h20" }
            path { "d": "M20 13V7l-5-5H6a2 2 0 0 0-2 2v9" }
            path { "d": "M6 20v-3" }
        }
    }
}
