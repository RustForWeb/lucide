use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct MicOffProps {
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
pub fn MicOff(props: MicOffProps) -> Element {
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
            path { "d": "M12 19v3" }
            path { "d": "M15 9.34V5a3 3 0 0 0-5.68-1.33" }
            path { "d": "M16.95 16.95A7 7 0 0 1 5 12v-2" }
            path { "d": "M18.89 13.23A7 7 0 0 0 19 12v-2" }
            path { "d": "m2 2 20 20" }
            path { "d": "M9 9v3a3 3 0 0 0 5.12 2.12" }
        }
    }
}
