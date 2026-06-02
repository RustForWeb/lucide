use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct ScaleProps {
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
pub fn Scale(props: ScaleProps) -> Element {
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
            path { "d": "M12 3v18" }
            path { "d": "m19 8 3 8a5 5 0 0 1-6 0zV7" }
            path { "d": "M3 7h1a17 17 0 0 0 8-2 17 17 0 0 0 8 2h1" }
            path { "d": "m5 8 3 8a5 5 0 0 1-6 0zV7" }
            path { "d": "M7 21h10" }
        }
    }
}
