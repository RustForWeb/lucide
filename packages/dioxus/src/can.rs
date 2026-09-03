use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct CanProps {
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
pub fn Can(props: CanProps) -> Element {
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
            path { "d": "M21 10.5a9 2.5 0 01-18 0v8a9 2.5 0 0018 0z" }
            path { "d": "M21 10.5A9 2.5 25.32 004.59 3.47 9 2.5 25.32 0021 10.5" }
            path { "d": "M3 10.5a9 2.5 0 016.527-2.405" }
            path { "d": "M9 16.858a31 31 0 006 0" }
        }
    }
}
