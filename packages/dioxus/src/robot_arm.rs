use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct RobotArmProps {
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
pub fn RobotArm(props: RobotArmProps) -> Element {
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
            path { "d": "M12 21 7.5 8.322" }
            path { "d": "m14 7 1.75-3.767a.5.5 0 0 1 .662-.172L20 5.005" }
            path { "d": "m20 8.998-3.588 1.944a.5.5 0 0 1-.662-.172L14 7H8" }
            path { "d": "M3.486 21h10" }
            path { "d": "M5 21V8.732" }
            circle { "cx": "6", "cy": "7", "r": "2" }
        }
    }
}
