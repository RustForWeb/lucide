use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct AlarmClockOffProps {
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
pub fn AlarmClockOff(props: AlarmClockOffProps) -> Element {
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
            path { "d": "M6.87 6.87a8 8 0 1 0 11.26 11.26" }
            path { "d": "M19.9 14.25a8 8 0 0 0-9.15-9.15" }
            path { "d": "m22 6-3-3" }
            path { "d": "M6.26 18.67 4 21" }
            path { "d": "m2 2 20 20" }
            path { "d": "M4 4 2 6" }
        }
    }
}
