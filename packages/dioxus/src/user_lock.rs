use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct UserLockProps {
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
pub fn UserLock(props: UserLockProps) -> Element {
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
            circle { "cx": "10", "cy": "7", "r": "4" }
            path { "d": "M10.3 15H7a4 4 0 0 0-4 4v2" }
            path { "d": "M15 15.5V14a2 2 0 0 1 4 0v1.5" }
            rect {
                "width": "8",
                "height": "5",
                "x": "13",
                "y": "16",
                "rx": ".899",
            }
        }
    }
}
