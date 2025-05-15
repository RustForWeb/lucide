use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct SlackProps {
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
pub fn Slack(props: SlackProps) -> Element {
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
            rect {
                "width": "3",
                "height": "8",
                "x": "13",
                "y": "2",
                "rx": "1.5",
            }
            path { "d": "M19 8.5V10h1.5A1.5 1.5 0 1 0 19 8.5" }
            rect {
                "width": "3",
                "height": "8",
                "x": "8",
                "y": "14",
                "rx": "1.5",
            }
            path { "d": "M5 15.5V14H3.5A1.5 1.5 0 1 0 5 15.5" }
            rect {
                "width": "8",
                "height": "3",
                "x": "14",
                "y": "13",
                "rx": "1.5",
            }
            path { "d": "M15.5 19H14v1.5a1.5 1.5 0 1 0 1.5-1.5" }
            rect {
                "width": "8",
                "height": "3",
                "x": "2",
                "y": "8",
                "rx": "1.5",
            }
            path { "d": "M8.5 5H10V3.5A1.5 1.5 0 1 0 8.5 5" }
        }
    }
}
