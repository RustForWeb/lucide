use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct TrainFrontTunnelProps {
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
pub fn TrainFrontTunnel(props: TrainFrontTunnelProps) -> Element {
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
            path { "d": "M2 22V12a10 10 0 1 1 20 0v10" }
            path { "d": "M15 6.8v1.4a3 2.8 0 1 1-6 0V6.8" }
            path { "d": "M10 15h.01" }
            path { "d": "M14 15h.01" }
            path { "d": "M10 19a4 4 0 0 1-4-4v-3a6 6 0 1 1 12 0v3a4 4 0 0 1-4 4Z" }
            path { "d": "m9 19-2 3" }
            path { "d": "m15 19 2 3" }
        }
    }
}
