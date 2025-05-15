use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct NotebookTextProps {
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
pub fn NotebookText(props: NotebookTextProps) -> Element {
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
            path { "d": "M2 6h4" }
            path { "d": "M2 10h4" }
            path { "d": "M2 14h4" }
            path { "d": "M2 18h4" }
            rect {
                "width": "16",
                "height": "20",
                "x": "4",
                "y": "2",
                "rx": "2",
            }
            path { "d": "M9.5 8h5" }
            path { "d": "M9.5 12H16" }
            path { "d": "M9.5 16H14" }
        }
    }
}
