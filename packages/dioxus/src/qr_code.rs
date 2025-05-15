use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct QrCodeProps {
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
pub fn QrCode(props: QrCodeProps) -> Element {
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
            rect {
                "width": "5",
                "height": "5",
                "x": "3",
                "y": "3",
                "rx": "1",
            }
            rect {
                "width": "5",
                "height": "5",
                "x": "16",
                "y": "3",
                "rx": "1",
            }
            rect {
                "width": "5",
                "height": "5",
                "x": "3",
                "y": "16",
                "rx": "1",
            }
            path { "d": "M21 16h-3a2 2 0 0 0-2 2v3" }
            path { "d": "M21 21v.01" }
            path { "d": "M12 7v3a2 2 0 0 1-2 2H7" }
            path { "d": "M3 12h.01" }
            path { "d": "M12 3h.01" }
            path { "d": "M12 16v.01" }
            path { "d": "M16 12h1" }
            path { "d": "M21 12v.01" }
            path { "d": "M12 21v-1" }
        }
    }
}
