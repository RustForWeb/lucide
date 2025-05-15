use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct ChartColumnStackedProps {
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
pub fn ChartColumnStacked(props: ChartColumnStackedProps) -> Element {
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
            path { "d": "M11 13H7" }
            path { "d": "M19 9h-4" }
            path { "d": "M3 3v16a2 2 0 0 0 2 2h16" }
            rect {
                "x": "15",
                "y": "5",
                "width": "4",
                "height": "12",
                "rx": "1",
            }
            rect {
                "x": "7",
                "y": "8",
                "width": "4",
                "height": "9",
                "rx": "1",
            }
        }
    }
}
