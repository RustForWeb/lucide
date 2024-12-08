use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct BatteryMediumProps {
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
pub fn BatteryMedium(props: BatteryMediumProps) -> Element {
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
            rect {
                "width": "16",
                "height": "10",
                "x": "2",
                "y": "7",
                "rx": "2",
                "ry": "2",
            }
            line {
                "x1": "22",
                "x2": "22",
                "y1": "11",
                "y2": "13",
            }
            line {
                "x1": "6",
                "x2": "6",
                "y1": "11",
                "y2": "13",
            }
            line {
                "x1": "10",
                "x2": "10",
                "y1": "11",
                "y2": "13",
            }
        }
    }
}
