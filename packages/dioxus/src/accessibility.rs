use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct AccessibilityProps {
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
pub fn Accessibility(props: AccessibilityProps) -> Element {
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
            circle { "cx": "16", "cy": "4", "r": "1" }
            path { "d": "m18 19 1-7-6 1" }
            path { "d": "m5 8 3-3 5.5 3-2.36 3.5" }
            path { "d": "M4.24 14.5a5 5 0 0 0 6.88 6" }
            path { "d": "M13.76 17.5a5 5 0 0 0-6.88-6" }
        }
    }
}
