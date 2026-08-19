use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct MidiPortProps {
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
pub fn MidiPort(props: MidiPortProps) -> Element {
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
            path { "d": "M12 18h.01" }
            path { "d": "M15 2.458V5a1 1 0 01-1 1h-4a1 1 0 01-1-1V2.458" }
            path { "d": "M16 16h.01" }
            path { "d": "M18 12h.01" }
            path { "d": "M6 12h.01" }
            path { "d": "M8 16h.01" }
            circle { "cx": "12", "cy": "12", "r": "10" }
        }
    }
}
