use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct BatteryChargingProps {
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
pub fn BatteryCharging(props: BatteryChargingProps) -> Element {
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
            path { "d": "M15 7h1a2 2 0 0 1 2 2v6a2 2 0 0 1-2 2h-2" }
            path { "d": "M6 7H4a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h1" }
            path { "d": "m11 7-3 5h4l-3 5" }
            line {
                "x1": "22",
                "x2": "22",
                "y1": "11",
                "y2": "13",
            }
        }
    }
}