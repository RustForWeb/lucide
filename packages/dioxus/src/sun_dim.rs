use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct SunDimProps {
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
pub fn SunDim(props: SunDimProps) -> Element {
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
            circle { "cx": "12", "cy": "12", "r": "4" }
            path { "d": "M12 4h.01" }
            path { "d": "M20 12h.01" }
            path { "d": "M12 20h.01" }
            path { "d": "M4 12h.01" }
            path { "d": "M17.657 6.343h.01" }
            path { "d": "M17.657 17.657h.01" }
            path { "d": "M6.343 17.657h.01" }
            path { "d": "M6.343 6.343h.01" }
        }
    }
}
