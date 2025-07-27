use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct CircleParkingOffProps {
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
pub fn CircleParkingOff(props: CircleParkingOffProps) -> Element {
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
            path { "d": "M12.656 7H13a3 3 0 0 1 2.984 3.307" }
            path { "d": "M13 13H9" }
            path { "d": "M19.071 19.071A1 1 0 0 1 4.93 4.93" }
            path { "d": "m2 2 20 20" }
            path { "d": "M8.357 2.687a10 10 0 0 1 12.956 12.956" }
            path { "d": "M9 17V9" }
        }
    }
}
