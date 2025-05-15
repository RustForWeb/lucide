use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct ChartCandlestickProps {
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
pub fn ChartCandlestick(props: ChartCandlestickProps) -> Element {
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
            path { "d": "M9 5v4" }
            rect {
                "width": "4",
                "height": "6",
                "x": "7",
                "y": "9",
                "rx": "1",
            }
            path { "d": "M9 15v2" }
            path { "d": "M17 3v2" }
            rect {
                "width": "4",
                "height": "8",
                "x": "15",
                "y": "5",
                "rx": "1",
            }
            path { "d": "M17 13v3" }
            path { "d": "M3 3v16a2 2 0 0 0 2 2h16" }
        }
    }
}
