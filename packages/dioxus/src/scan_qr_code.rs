use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct ScanQrCodeProps {
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
pub fn ScanQrCode(props: ScanQrCodeProps) -> Element {
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
            path { "d": "M17 12v4a1 1 0 0 1-1 1h-4" }
            path { "d": "M17 3h2a2 2 0 0 1 2 2v2" }
            path { "d": "M17 8V7" }
            path { "d": "M21 17v2a2 2 0 0 1-2 2h-2" }
            path { "d": "M3 7V5a2 2 0 0 1 2-2h2" }
            path { "d": "M7 17h.01" }
            path { "d": "M7 21H5a2 2 0 0 1-2-2v-2" }
            rect {
                "x": "7",
                "y": "7",
                "width": "5",
                "height": "5",
                "rx": "1",
            }
        }
    }
}
