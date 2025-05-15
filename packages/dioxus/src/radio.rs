use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct RadioProps {
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
pub fn Radio(props: RadioProps) -> Element {
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
            path { "d": "M4.9 19.1C1 15.2 1 8.8 4.9 4.9" }
            path { "d": "M7.8 16.2c-2.3-2.3-2.3-6.1 0-8.5" }
            circle { "cx": "12", "cy": "12", "r": "2" }
            path { "d": "M16.2 7.8c2.3 2.3 2.3 6.1 0 8.5" }
            path { "d": "M19.1 4.9C23 8.8 23 15.1 19.1 19" }
        }
    }
}
