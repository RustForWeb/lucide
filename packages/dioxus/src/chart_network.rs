use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct ChartNetworkProps {
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
pub fn ChartNetwork(props: ChartNetworkProps) -> Element {
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
            path { "d": "m13.11 7.664 1.78 2.672" }
            path { "d": "m14.162 12.788-3.324 1.424" }
            path { "d": "m20 4-6.06 1.515" }
            path { "d": "M3 3v16a2 2 0 0 0 2 2h16" }
            circle { "cx": "12", "cy": "6", "r": "2" }
            circle { "cx": "16", "cy": "12", "r": "2" }
            circle { "cx": "9", "cy": "15", "r": "2" }
        }
    }
}
