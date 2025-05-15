use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct SquareSplitVerticalProps {
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
pub fn SquareSplitVertical(props: SquareSplitVerticalProps) -> Element {
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
            path { "d": "M5 8V5c0-1 1-2 2-2h10c1 0 2 1 2 2v3" }
            path { "d": "M19 16v3c0 1-1 2-2 2H7c-1 0-2-1-2-2v-3" }
            line {
                "x1": "4",
                "x2": "20",
                "y1": "12",
                "y2": "12",
            }
        }
    }
}
