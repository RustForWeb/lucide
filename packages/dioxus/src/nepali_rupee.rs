use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct NepaliRupeeProps {
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
pub fn NepaliRupee(props: NepaliRupeeProps) -> Element {
    let stroke_width = if props.absolute_stroke_width {
        props.stroke_width * 24 / props.size
    } else {
        props.stroke_width
    };
    rsx! {
        svg {
            "xmlns": "http://www.w3.org/2000/svg",
            "class": if let Some(class) = props.class { class },
            "style": if let Some(style) = props.style { style },
            "width": "{props.size}",
            "height": "{props.size}",
            "viewBox": "0 0 24 24",
            "fill": "{props.fill}",
            "stroke": "{props.color}",
            "stroke-width": "{stroke_width}",
            "stroke-linecap": "round",
            "stroke-linejoin": "round",
            path { "d": "M18 16.173 A4.74 4.74 0 0 0 13.496 8.005" }
            path { "d": "M4 3 L20 3" }
            path { "d": "M5 13 L13.5 21" }
            path { "d": "M5 13 L9 13" }
            path { "d": "M8 13 C15.5 13 14.667 3 8 3" }
        }
    }
}
