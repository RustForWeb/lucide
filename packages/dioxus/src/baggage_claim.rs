use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct BaggageClaimProps {
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
pub fn BaggageClaim(props: BaggageClaimProps) -> Element {
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
            path { "d": "M22 18H6a2 2 0 0 1-2-2V7a2 2 0 0 0-2-2" }
            path { "d": "M17 14V4a2 2 0 0 0-2-2h-1a2 2 0 0 0-2 2v10" }
            rect {
                "width": "13",
                "height": "8",
                "x": "8",
                "y": "6",
                "rx": "1",
            }
            circle { "cx": "18", "cy": "20", "r": "2" }
            circle { "cx": "9", "cy": "20", "r": "2" }
        }
    }
}
