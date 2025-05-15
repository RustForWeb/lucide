use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct ShipWheelProps {
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
pub fn ShipWheel(props: ShipWheelProps) -> Element {
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
            circle { "cx": "12", "cy": "12", "r": "8" }
            path { "d": "M12 2v7.5" }
            path { "d": "m19 5-5.23 5.23" }
            path { "d": "M22 12h-7.5" }
            path { "d": "m19 19-5.23-5.23" }
            path { "d": "M12 14.5V22" }
            path { "d": "M10.23 13.77 5 19" }
            path { "d": "M9.5 12H2" }
            path { "d": "M10.23 10.23 5 5" }
            circle { "cx": "12", "cy": "12", "r": "2.5" }
        }
    }
}
