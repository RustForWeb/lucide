use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct EyeDashedProps {
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
pub fn EyeDashed(props: EyeDashedProps) -> Element {
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
            path { "d": "M13.054 18.946a11 11 0 0 1-2.11 0" }
            path { "d": "M13.054 5.054a11 11 0 0 0-2.11-.001" }
            path { "d": "M17.072 6.274a11 11 0 0 1 1.753 1.173" }
            path { "d": "M18.825 16.552a11 11 0 0 1-1.753 1.174" }
            path { "d": "M2.514 13.303a11 11 0 0 1-.452-.954 1 1 0 0 1 0-.697 11 11 0 0 1 .45-.955" }
            path { "d": "M21.485 10.697a11 11 0 0 1 .453.955 1 1 0 0 1 0 .697 11 11 0 0 1-.453.954" }
            path { "d": "M5.173 7.448a11 11 0 0 1 1.753-1.174" }
            path { "d": "M6.926 17.726a11 11 0 0 1-1.753-1.174" }
            circle { "cx": "12", "cy": "12", "r": "3" }
        }
    }
}
