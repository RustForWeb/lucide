use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct BookKeyProps {
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
pub fn BookKey(props: BookKeyProps) -> Element {
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
            path { "d": "M13 2H6.5A2.5 2.5 0 0 0 4 4.5v15" }
            path { "d": "M17 2v6" }
            path { "d": "M17 4h2" }
            path { "d": "M20 15.2V21a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20" }
            circle { "cx": "17", "cy": "10", "r": "2" }
        }
    }
}
