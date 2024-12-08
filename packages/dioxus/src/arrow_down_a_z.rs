use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct ArrowDownAZProps {
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
pub fn ArrowDownAZ(props: ArrowDownAZProps) -> Element {
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
            path { "d": "m3 16 4 4 4-4" }
            path { "d": "M7 20V4" }
            path { "d": "M20 8h-5" }
            path { "d": "M15 10V6.5a2.5 2.5 0 0 1 5 0V10" }
            path { "d": "M15 14h5l-5 6h5" }
        }
    }
}
