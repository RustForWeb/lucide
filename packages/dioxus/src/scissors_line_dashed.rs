use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct ScissorsLineDashedProps {
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
pub fn ScissorsLineDashed(props: ScissorsLineDashedProps) -> Element {
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
            path { "d": "M5.42 9.42 8 12" }
            circle { "cx": "4", "cy": "8", "r": "2" }
            path { "d": "m14 6-8.58 8.58" }
            circle { "cx": "4", "cy": "16", "r": "2" }
            path { "d": "M10.8 14.8 14 18" }
            path { "d": "M16 12h-2" }
            path { "d": "M22 12h-2" }
        }
    }
}
