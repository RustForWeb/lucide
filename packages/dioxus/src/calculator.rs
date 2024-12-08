use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct CalculatorProps {
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
pub fn Calculator(props: CalculatorProps) -> Element {
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
                "height": "20",
                "x": "4",
                "y": "2",
                "rx": "2",
            }
            line {
                "x1": "8",
                "x2": "16",
                "y1": "6",
                "y2": "6",
            }
            line {
                "x1": "16",
                "x2": "16",
                "y1": "14",
                "y2": "18",
            }
            path { "d": "M16 10h.01" }
            path { "d": "M12 10h.01" }
            path { "d": "M8 10h.01" }
            path { "d": "M12 14h.01" }
            path { "d": "M8 14h.01" }
            path { "d": "M12 18h.01" }
            path { "d": "M8 18h.01" }
        }
    }
}
