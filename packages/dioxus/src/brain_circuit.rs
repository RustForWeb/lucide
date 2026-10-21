use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct BrainCircuitProps {
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
pub fn BrainCircuit(props: BrainCircuitProps) -> Element {
    let stroke_width = if props.absolute_stroke_width {
        props.stroke_width * 24 / props.size
    } else {
        props.stroke_width
    };
    rsx! {
        svg {
            "xmlns": "http://www.w3.org/2000/svg",
            "class": if let Some(class) = props.class { class },
            "style": if let Some(style) = props.style { style },
            "width": "{props.size}",
            "height": "{props.size}",
            "viewBox": "0 0 24 24",
            "fill": "{props.fill}",
            "stroke": "{props.color}",
            "stroke-width": "{stroke_width}",
            "stroke-linecap": "round",
            "stroke-linejoin": "round",
            path { "d": "M12 13h4" }
            path { "d": "M12 18h6a2 2 0 012 2v1" }
            path { "d": "M12 8h8" }
            path { "d": "M12 9a4.17 4.17 0 01-3 4" }
            path { "d": "M16 8V5a2 2 0 012-2" }
            path { "d": "M6 18a4 4 0 01-2-7.465" }
            path { "d": "M6.004 5.125a4 4 0 00-2.526 5.77" }
            path { "d": "M6.294 6.294A3 3 0 1112 5v13a4 4 0 11-7.967-.518" }
            circle {
                "cx": "16",
                "cy": "13",
                "r": "0.5",
                "fill": "currentColor",
            }
            circle {
                "cx": "18",
                "cy": "3",
                "r": "0.5",
                "fill": "currentColor",
            }
            circle {
                "cx": "20",
                "cy": "21",
                "r": "0.5",
                "fill": "currentColor",
            }
            circle {
                "cx": "20",
                "cy": "8",
                "r": "0.5",
                "fill": "currentColor",
            }
        }
    }
}
