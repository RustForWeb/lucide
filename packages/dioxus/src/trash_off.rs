use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct TrashOffProps {
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
pub fn TrashOff(props: TrashOffProps) -> Element {
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
            path { "d": "M10 11v6" }
            path { "d": "M14 17v-3" }
            path { "d": "M16 6V4a2 2 0 00-2-2h-4a2 2 0 00-1.576.768" }
            path { "d": "M19 6v7.344" }
            path { "d": "m2 2 20 20" }
            path { "d": "M21 6h-9.344" }
            path { "d": "M3 6h3" }
            path { "d": "M5 6v14a2 2 0 002 2h10a2 2 0 002-2v-1" }
        }
    }
}
