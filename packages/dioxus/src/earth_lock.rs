use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct EarthLockProps {
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
pub fn EarthLock(props: EarthLockProps) -> Element {
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
            path { "d": "M7 3.34V5a3 3 0 0 0 3 3" }
            path { "d": "M11 21.95V18a2 2 0 0 0-2-2 2 2 0 0 1-2-2v-1a2 2 0 0 0-2-2H2.05" }
            path { "d": "M21.54 15H17a2 2 0 0 0-2 2v4.54" }
            path { "d": "M12 2a10 10 0 1 0 9.54 13" }
            path { "d": "M20 6V4a2 2 0 1 0-4 0v2" }
            rect {
                "width": "8",
                "height": "5",
                "x": "14",
                "y": "6",
                "rx": "1",
            }
        }
    }
}
