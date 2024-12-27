use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct CalendarSyncProps {
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
pub fn CalendarSync(props: CalendarSyncProps) -> Element {
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
            path { "d": "M11 10v4h4" }
            path { "d": "m11 14 1.535-1.605a5 5 0 0 1 8 1.5" }
            path { "d": "M16 2v4" }
            path { "d": "m21 18-1.535 1.605a5 5 0 0 1-8-1.5" }
            path { "d": "M21 22v-4h-4" }
            path { "d": "M21 8.5V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h4.3" }
            path { "d": "M3 10h4" }
            path { "d": "M8 2v4" }
        }
    }
}
