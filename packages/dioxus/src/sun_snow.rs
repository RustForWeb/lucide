use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct SunSnowProps {
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
pub fn SunSnow(props: SunSnowProps) -> Element {
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
            path { "d": "M10 21v-1" }
            path { "d": "M10 4V3" }
            path { "d": "M10 9a3 3 0 0 0 0 6" }
            path { "d": "m14 20 1.25-2.5L18 18" }
            path { "d": "m14 4 1.25 2.5L18 6" }
            path { "d": "m17 21-3-6 1.5-3H22" }
            path { "d": "m17 3-3 6 1.5 3" }
            path { "d": "M2 12h1" }
            path { "d": "m20 10-1.5 2 1.5 2" }
            path { "d": "m3.64 18.36.7-.7" }
            path { "d": "m4.34 6.34-.7-.7" }
        }
    }
}
