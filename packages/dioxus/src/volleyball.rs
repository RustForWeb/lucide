use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct VolleyballProps {
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
pub fn Volleyball(props: VolleyballProps) -> Element {
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
            path { "d": "M11.1 7.1a16.55 16.55 0 0 1 10.9 4" }
            path { "d": "M12 12a12.6 12.6 0 0 1-8.7 5" }
            path { "d": "M16.8 13.6a16.55 16.55 0 0 1-9 7.5" }
            path { "d": "M20.7 17a12.8 12.8 0 0 0-8.7-5 13.3 13.3 0 0 1 0-10" }
            path { "d": "M6.3 3.8a16.55 16.55 0 0 0 1.9 11.5" }
            circle { "cx": "12", "cy": "12", "r": "10" }
        }
    }
}
