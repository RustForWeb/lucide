use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct DnaOffProps {
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
pub fn DnaOff(props: DnaOffProps) -> Element {
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
            path { "d": "M15 2c-1.35 1.5-2.092 3-2.5 4.5L14 8" }
            path { "d": "m17 6-2.891-2.891" }
            path { "d": "M2 15c3.333-3 6.667-3 10-3" }
            path { "d": "m2 2 20 20" }
            path { "d": "m20 9 .891.891" }
            path { "d": "M22 9c-1.5 1.35-3 2.092-4.5 2.5l-1-1" }
            path { "d": "M3.109 14.109 4 15" }
            path { "d": "m6.5 12.5 1 1" }
            path { "d": "m7 18 2.891 2.891" }
            path { "d": "M9 22c1.35-1.5 2.092-3 2.5-4.5L10 16" }
        }
    }
}
