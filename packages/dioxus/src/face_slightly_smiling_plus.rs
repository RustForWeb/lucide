use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct FaceSlightlySmilingPlusProps {
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
pub fn FaceSlightlySmilingPlus(props: FaceSlightlySmilingPlusProps) -> Element {
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
            path { "d": "M13.267 2.08a10 10 0 108.653 8.653" }
            path { "d": "M15 10V9" }
            path { "d": "M16 5h6" }
            path { "d": "M16.472 15a6 6 0 01-8.943 0" }
            path { "d": "M19 2v6" }
            path { "d": "M9 10V9" }
        }
    }
}
