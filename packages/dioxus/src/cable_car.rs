use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct CableCarProps {
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
pub fn CableCar(props: CableCarProps) -> Element {
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
            path { "d": "M10 3h.01" }
            path { "d": "M14 2h.01" }
            path { "d": "m2 9 20-5" }
            path { "d": "M12 12V6.5" }
            rect {
                "width": "16",
                "height": "10",
                "x": "4",
                "y": "12",
                "rx": "3",
            }
            path { "d": "M9 12v5" }
            path { "d": "M15 12v5" }
            path { "d": "M4 17h16" }
        }
    }
}
