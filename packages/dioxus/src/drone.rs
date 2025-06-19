use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct DroneProps {
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
pub fn Drone(props: DroneProps) -> Element {
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
            path { "d": "M10 10 7 7" }
            path { "d": "m10 14-3 3" }
            path { "d": "m14 10 3-3" }
            path { "d": "m14 14 3 3" }
            path { "d": "M14.205 4.139a4 4 0 1 1 5.439 5.863" }
            path { "d": "M19.637 14a4 4 0 1 1-5.432 5.868" }
            path { "d": "M4.367 10a4 4 0 1 1 5.438-5.862" }
            path { "d": "M9.795 19.862a4 4 0 1 1-5.429-5.873" }
            rect {
                "x": "10",
                "y": "8",
                "width": "4",
                "height": "8",
                "rx": "1",
            }
        }
    }
}
