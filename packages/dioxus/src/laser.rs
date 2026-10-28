use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct LaserProps {
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
pub fn Laser(props: LaserProps) -> Element {
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
            path { "d": "m13 8 .95-.95" }
            path { "d": "M13.95 16.95 13 16" }
            path { "d": "M22 12h-9" }
            path { "d": "M4 12H2" }
            path { "d": "M4.05 16.95 5 16" }
            path { "d": "m5 8-.95-.95" }
            path { "d": "M9 12h.01" }
            path { "d": "M9 19v-2" }
            path { "d": "M9 7V5" }
        }
    }
}
