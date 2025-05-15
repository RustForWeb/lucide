use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct RefreshCcwDotProps {
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
pub fn RefreshCcwDot(props: RefreshCcwDotProps) -> Element {
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
            path { "d": "M3 2v6h6" }
            path { "d": "M21 12A9 9 0 0 0 6 5.3L3 8" }
            path { "d": "M21 22v-6h-6" }
            path { "d": "M3 12a9 9 0 0 0 15 6.7l3-2.7" }
            circle { "cx": "12", "cy": "12", "r": "1" }
        }
    }
}
