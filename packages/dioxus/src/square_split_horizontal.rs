use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct SquareSplitHorizontalProps {
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
pub fn SquareSplitHorizontal(props: SquareSplitHorizontalProps) -> Element {
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
            path { "d": "M8 19H5c-1 0-2-1-2-2V7c0-1 1-2 2-2h3" }
            path { "d": "M16 5h3c1 0 2 1 2 2v10c0 1-1 2-2 2h-3" }
            line {
                "x1": "12",
                "x2": "12",
                "y1": "4",
                "y2": "20",
            }
        }
    }
}
