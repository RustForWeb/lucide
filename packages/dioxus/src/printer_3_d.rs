use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct Printer3DProps {
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
pub fn Printer3D(props: Printer3DProps) -> Element {
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
            path { "d": "M10 11v1" }
            path { "d": "M12 8h8" }
            path { "d": "M15 20v-3a1 1 0 00-1-1H9a1 1 0 00-1 1v3" }
            path { "d": "M4 20h16" }
            path { "d": "M4 22V4a2 2 0 012-2h12a2 2 0 012 2v18" }
            path { "d": "M4 8h4" }
            path { "d": "M8.635 10.093A2 2 0 018 8.631V7a1 1 0 011-1h2a1 1 0 011 1v2a1 1 0 01-.293.707l-1 1a1 1 0 01-1.414 0z" }
        }
    }
}
