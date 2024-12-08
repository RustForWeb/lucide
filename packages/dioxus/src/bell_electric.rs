use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct BellElectricProps {
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
pub fn BellElectric(props: BellElectricProps) -> Element {
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
            path { "d": "M18.8 4A6.3 8.7 0 0 1 20 9" }
            path { "d": "M9 9h.01" }
            circle { "cx": "9", "cy": "9", "r": "7" }
            rect {
                "width": "10",
                "height": "6",
                "x": "4",
                "y": "16",
                "rx": "2",
            }
            path { "d": "M14 19c3 0 4.6-1.6 4.6-1.6" }
            circle { "cx": "20", "cy": "16", "r": "2" }
        }
    }
}
