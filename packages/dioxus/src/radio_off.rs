use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct RadioOffProps {
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
pub fn RadioOff(props: RadioOffProps) -> Element {
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
            path { "d": "M13.414 13.414a2 2 0 1 1-2.828-2.828" }
            path { "d": "M16.247 7.761a6 6 0 0 1 1.744 4.572" }
            path { "d": "M19.075 4.933a10 10 0 0 1 2.234 10.72" }
            path { "d": "m2 2 20 20" }
            path { "d": "M4.925 19.067a10 10 0 0 1 0-14.134" }
            path { "d": "M7.753 16.239a6 6 0 0 1 0-8.478" }
        }
    }
}
