use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct MonitorSpeakerProps {
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
}
#[component]
pub fn MonitorSpeaker(props: MonitorSpeakerProps) -> Element {
    let stroke_width = if props.absolute_stroke_width {
        props.stroke_width * 24 / props.size
    } else {
        props.stroke_width
    };
    rsx! {
        svg {
            "xmlns": "http://www.w3.org/2000/svg",
            "class": if let Some(class) = props.class { "{class}" },
            "width": "{props.size}",
            "height": "{props.size}",
            "viewBox": "0 0 24 24",
            "fill": "{props.fill}",
            "stroke": "{props.color}",
            "stroke-width": "{stroke_width}",
            "stroke-linecap": "round",
            "stroke-linejoin": "round",
            path { "d": "M5.5 20H8" }
            path { "d": "M17 9h.01" }
            rect {
                "width": "10",
                "height": "16",
                "x": "12",
                "y": "4",
                "rx": "2",
            }
            path { "d": "M8 6H4a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h4" }
            circle { "cx": "17", "cy": "15", "r": "1" }
        }
    }
}
