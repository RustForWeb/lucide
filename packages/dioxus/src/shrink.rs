use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct ShrinkProps {
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
pub fn Shrink(props: ShrinkProps) -> Element {
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
            path { "d": "m15 15 6 6m-6-6v4.8m0-4.8h4.8" }
            path { "d": "M9 19.8V15m0 0H4.2M9 15l-6 6" }
            path { "d": "M15 4.2V9m0 0h4.8M15 9l6-6" }
            path { "d": "M9 4.2V9m0 0H4.2M9 9 3 3" }
        }
    }
}
