use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct CalendarCogProps {
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
pub fn CalendarCog(props: CalendarCogProps) -> Element {
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
            path { "d": "m15.2 16.9-.9-.4" }
            path { "d": "m15.2 19.1-.9.4" }
            path { "d": "M16 2v4" }
            path { "d": "m16.9 15.2-.4-.9" }
            path { "d": "m16.9 20.8-.4.9" }
            path { "d": "m19.5 14.3-.4.9" }
            path { "d": "m19.5 21.7-.4-.9" }
            path { "d": "M21 10.5V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h6" }
            path { "d": "m21.7 16.5-.9.4" }
            path { "d": "m21.7 19.5-.9-.4" }
            path { "d": "M3 10h18" }
            path { "d": "M8 2v4" }
            circle { "cx": "18", "cy": "18", "r": "3" }
        }
    }
}
