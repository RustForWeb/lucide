use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct RadarProps {
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
pub fn Radar(props: RadarProps) -> Element {
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
            path { "d": "M19.07 4.93A10 10 0 0 0 6.99 3.34" }
            path { "d": "M4 6h.01" }
            path { "d": "M2.29 9.62A10 10 0 1 0 21.31 8.35" }
            path { "d": "M16.24 7.76A6 6 0 1 0 8.23 16.67" }
            path { "d": "M12 18h.01" }
            path { "d": "M17.99 11.66A6 6 0 0 1 15.77 16.67" }
            circle { "cx": "12", "cy": "12", "r": "2" }
            path { "d": "m13.41 10.59 5.66-5.66" }
        }
    }
}
