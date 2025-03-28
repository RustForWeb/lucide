use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct TextCursorInputProps {
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
pub fn TextCursorInput(props: TextCursorInputProps) -> Element {
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
            path { "d": "M5 4h1a3 3 0 0 1 3 3 3 3 0 0 1 3-3h1" }
            path { "d": "M13 20h-1a3 3 0 0 1-3-3 3 3 0 0 1-3 3H5" }
            path { "d": "M5 16H4a2 2 0 0 1-2-2v-4a2 2 0 0 1 2-2h1" }
            path { "d": "M13 8h7a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2h-7" }
            path { "d": "M9 7v10" }
        }
    }
}
