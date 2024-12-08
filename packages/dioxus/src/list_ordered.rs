use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct ListOrderedProps {
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
pub fn ListOrdered(props: ListOrderedProps) -> Element {
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
            path { "d": "M10 12h11" }
            path { "d": "M10 18h11" }
            path { "d": "M10 6h11" }
            path { "d": "M4 10h2" }
            path { "d": "M4 6h1v4" }
            path { "d": "M6 18H4c0-1 2-2 2-3s-1-1.5-2-1" }
        }
    }
}
