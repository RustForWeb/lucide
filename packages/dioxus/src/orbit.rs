use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct OrbitProps {
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
}
#[component]
pub fn Orbit(props: OrbitProps) -> Element {
    let stroke_width = if props.absolute_stroke_width {
        props.stroke_width * 24 / props.size
    } else {
        props.stroke_width
    };
    rsx! {
        svg {
            "xmlns": "http://www.w3.org/2000/svg",
            "class": if let Some(class) = props.class { "{class}" },
            "width": "{props.size}",
            "height": "{props.size}",
            "viewBox": "0 0 24 24",
            "fill": "{props.fill}",
            "stroke": "{props.color}",
            "stroke-width": "{stroke_width}",
            "stroke-linecap": "round",
            "stroke-linejoin": "round",
            circle { "cx": "12", "cy": "12", "r": "3" }
            circle { "cx": "19", "cy": "5", "r": "2" }
            circle { "cx": "5", "cy": "19", "r": "2" }
            path { "d": "M10.4 21.9a10 10 0 0 0 9.941-15.416" }
            path { "d": "M13.5 2.1a10 10 0 0 0-9.841 15.416" }
        }
    }
}
