use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct SquareBottomDashedScissorsProps {
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
pub fn SquareBottomDashedScissors(props: SquareBottomDashedScissorsProps) -> Element {
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
            path { "d": "M4 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v16a2 2 0 0 1-2 2" }
            path { "d": "M10 22H8" }
            path { "d": "M16 22h-2" }
            circle { "cx": "8", "cy": "8", "r": "2" }
            path { "d": "M9.414 9.414 12 12" }
            path { "d": "M14.8 14.8 18 18" }
            circle { "cx": "8", "cy": "16", "r": "2" }
            path { "d": "m18 6-8.586 8.586" }
        }
    }
}
