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
            path { "d": "M14 21h1" }
            path { "d": "m17 17-2.18-2.18" }
            path { "d": "M5 21a2 2 0 01-2-2V5a2 2 0 012-2h14a2 2 0 012 2v14a2 2 0 01-2 2" }
            path { "d": "M9 21h1" }
            path { "d": "M9.56 14.44 17 7" }
            path { "d": "M9.56 9.56 12 12" }
            circle { "cx": "8.5", "cy": "15.5", "r": "1.5" }
            circle { "cx": "8.5", "cy": "8.5", "r": "1.5" }
        }
    }
}
