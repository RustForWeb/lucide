use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct FerrisWheelProps {
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
pub fn FerrisWheel(props: FerrisWheelProps) -> Element {
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
            circle { "cx": "12", "cy": "12", "r": "2" }
            path { "d": "M12 2v4" }
            path { "d": "m6.8 15-3.5 2" }
            path { "d": "m20.7 7-3.5 2" }
            path { "d": "M6.8 9 3.3 7" }
            path { "d": "m20.7 17-3.5-2" }
            path { "d": "m9 22 3-8 3 8" }
            path { "d": "M8 22h8" }
            path { "d": "M18 18.7a9 9 0 1 0-12 0" }
        }
    }
}
