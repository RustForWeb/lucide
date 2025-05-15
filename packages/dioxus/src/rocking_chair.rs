use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct RockingChairProps {
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
pub fn RockingChair(props: RockingChairProps) -> Element {
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
            polyline { "points": "3.5 2 6.5 12.5 18 12.5" }
            line {
                "x1": "9.5",
                "x2": "5.5",
                "y1": "12.5",
                "y2": "20",
            }
            line {
                "x1": "15",
                "x2": "18.5",
                "y1": "12.5",
                "y2": "20",
            }
            path { "d": "M2.75 18a13 13 0 0 0 18.5 0" }
        }
    }
}
