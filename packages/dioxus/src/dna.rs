use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct DnaProps {
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
pub fn Dna(props: DnaProps) -> Element {
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
            path { "d": "m10 16 1.5 1.5" }
            path { "d": "m14 8-1.5-1.5" }
            path { "d": "M15 2c-1.798 1.998-2.518 3.995-2.807 5.993" }
            path { "d": "m16.5 10.5 1 1" }
            path { "d": "m17 6-2.891-2.891" }
            path { "d": "M2 15c6.667-6 13.333 0 20-6" }
            path { "d": "m20 9 .891.891" }
            path { "d": "M3.109 14.109 4 15" }
            path { "d": "m6.5 12.5 1 1" }
            path { "d": "m7 18 2.891 2.891" }
            path { "d": "M9 22c1.798-1.998 2.518-3.995 2.807-5.993" }
        }
    }
}
