use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct BatteryPlusProps {
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
pub fn BatteryPlus(props: BatteryPlusProps) -> Element {
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
            path { "d": "M10 9v6" }
            path { "d": "M13.5 7H16a2 2 0 0 1 2 2v6a2 2 0 0 1-2 2h-2.5" }
            path { "d": "M22 11v2" }
            path { "d": "M6.5 17H4a2 2 0 0 1-2-2V9a2 2 0 0 1 2-2h2.5" }
            path { "d": "M7 12h6" }
        }
    }
}
