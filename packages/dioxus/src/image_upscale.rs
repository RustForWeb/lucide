use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct ImageUpscaleProps {
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
pub fn ImageUpscale(props: ImageUpscaleProps) -> Element {
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
            path { "d": "M16 3h5v5" }
            path { "d": "M17 21h2a2 2 0 0 0 2-2" }
            path { "d": "M21 12v3" }
            path { "d": "m21 3-5 5" }
            path { "d": "M3 7V5a2 2 0 0 1 2-2" }
            path { "d": "m5 21 4.144-4.144a1.21 1.21 0 0 1 1.712 0L13 19" }
            path { "d": "M9 3h3" }
            rect {
                "x": "3",
                "y": "11",
                "width": "10",
                "height": "10",
                "rx": "1",
            }
        }
    }
}
