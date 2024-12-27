use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct BookDashedProps {
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
pub fn BookDashed(props: BookDashedProps) -> Element {
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
            path { "d": "M12 17h1.5" }
            path { "d": "M12 22h1.5" }
            path { "d": "M12 2h1.5" }
            path { "d": "M17.5 22H19a1 1 0 0 0 1-1" }
            path { "d": "M17.5 2H19a1 1 0 0 1 1 1v1.5" }
            path { "d": "M20 14v3h-2.5" }
            path { "d": "M20 8.5V10" }
            path { "d": "M4 10V8.5" }
            path { "d": "M4 19.5V14" }
            path { "d": "M4 4.5A2.5 2.5 0 0 1 6.5 2H8" }
            path { "d": "M8 22H6.5a1 1 0 0 1 0-5H8" }
        }
    }
}
