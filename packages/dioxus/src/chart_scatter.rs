use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct ChartScatterProps {
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
pub fn ChartScatter(props: ChartScatterProps) -> Element {
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
            circle {
                "cx": "7.5",
                "cy": "7.5",
                "r": ".5",
                "fill": "currentColor",
            }
            circle {
                "cx": "18.5",
                "cy": "5.5",
                "r": ".5",
                "fill": "currentColor",
            }
            circle {
                "cx": "11.5",
                "cy": "11.5",
                "r": ".5",
                "fill": "currentColor",
            }
            circle {
                "cx": "7.5",
                "cy": "16.5",
                "r": ".5",
                "fill": "currentColor",
            }
            circle {
                "cx": "17.5",
                "cy": "14.5",
                "r": ".5",
                "fill": "currentColor",
            }
            path { "d": "M3 3v16a2 2 0 0 0 2 2h16" }
        }
    }
}
