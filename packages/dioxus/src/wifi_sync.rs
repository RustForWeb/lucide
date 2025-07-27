use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct WifiSyncProps {
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
pub fn WifiSync(props: WifiSyncProps) -> Element {
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
            path { "d": "M11.965 10.105v4L13.5 12.5a5 5 0 0 1 8 1.5" }
            path { "d": "M11.965 14.105h4" }
            path { "d": "M17.965 18.105h4L20.43 19.71a5 5 0 0 1-8-1.5" }
            path { "d": "M2 8.82a15 15 0 0 1 20 0" }
            path { "d": "M21.965 22.105v-4" }
            path { "d": "M5 12.86a10 10 0 0 1 3-2.032" }
            path { "d": "M8.5 16.429h.01" }
        }
    }
}
