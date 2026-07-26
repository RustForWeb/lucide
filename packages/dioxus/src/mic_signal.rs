use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct MicSignalProps {
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
pub fn MicSignal(props: MicSignalProps) -> Element {
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
            path { "d": "M12 17v4" }
            path { "d": "M18 11a6 6 0 00-3-5.197" }
            path { "d": "M2 11a10 10 0 015-8.662" }
            path { "d": "M22 11a10 10 0 00-5-8.662" }
            path { "d": "M6 11a6 6 0 013-5.197" }
            path { "d": "M9 21h6" }
            rect {
                "x": "10",
                "y": "9",
                "width": "4",
                "height": "8",
                "rx": "2",
            }
        }
    }
}
