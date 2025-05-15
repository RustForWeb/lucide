use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct ImagesProps {
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
pub fn Images(props: ImagesProps) -> Element {
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
            path { "d": "M18 22H4a2 2 0 0 1-2-2V6" }
            path { "d": "m22 13-1.296-1.296a2.41 2.41 0 0 0-3.408 0L11 18" }
            circle { "cx": "12", "cy": "8", "r": "2" }
            rect {
                "width": "16",
                "height": "16",
                "x": "6",
                "y": "2",
                "rx": "2",
            }
        }
    }
}
