use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct LocateOffProps {
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
pub fn LocateOff(props: LocateOffProps) -> Element {
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
            path { "d": "M12 19v3" }
            path { "d": "M12 2v3" }
            path { "d": "M18.89 13.24a7 7 0 0 0-8.13-8.13" }
            path { "d": "M19 12h3" }
            path { "d": "M2 12h3" }
            path { "d": "m2 2 20 20" }
            path { "d": "M7.05 7.05a7 7 0 0 0 9.9 9.9" }
        }
    }
}
