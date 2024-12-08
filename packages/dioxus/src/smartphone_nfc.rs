use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct SmartphoneNfcProps {
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
pub fn SmartphoneNfc(props: SmartphoneNfcProps) -> Element {
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
            rect {
                "width": "7",
                "height": "12",
                "x": "2",
                "y": "6",
                "rx": "1",
            }
            path { "d": "M13 8.32a7.43 7.43 0 0 1 0 7.36" }
            path { "d": "M16.46 6.21a11.76 11.76 0 0 1 0 11.58" }
            path { "d": "M19.91 4.1a15.91 15.91 0 0 1 .01 15.8" }
        }
    }
}
