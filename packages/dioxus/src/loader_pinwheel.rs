use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct LoaderPinwheelProps {
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
pub fn LoaderPinwheel(props: LoaderPinwheelProps) -> Element {
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
            path { "d": "M22 12a1 1 0 0 1-10 0 1 1 0 0 0-10 0" }
            path { "d": "M7 20.7a1 1 0 1 1 5-8.7 1 1 0 1 0 5-8.6" }
            path { "d": "M7 3.3a1 1 0 1 1 5 8.6 1 1 0 1 0 5 8.6" }
            circle { "cx": "12", "cy": "12", "r": "10" }
        }
    }
}
