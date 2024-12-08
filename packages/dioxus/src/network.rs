use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct NetworkProps {
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
pub fn Network(props: NetworkProps) -> Element {
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
            rect {
                "x": "16",
                "y": "16",
                "width": "6",
                "height": "6",
                "rx": "1",
            }
            rect {
                "x": "2",
                "y": "16",
                "width": "6",
                "height": "6",
                "rx": "1",
            }
            rect {
                "x": "9",
                "y": "2",
                "width": "6",
                "height": "6",
                "rx": "1",
            }
            path { "d": "M5 16v-3a1 1 0 0 1 1-1h12a1 1 0 0 1 1 1v3" }
            path { "d": "M12 12V8" }
        }
    }
}
