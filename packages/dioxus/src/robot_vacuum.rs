use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct RobotVacuumProps {
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
pub fn RobotVacuum(props: RobotVacuumProps) -> Element {
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
            path { "d": "M11 17h2" }
            path { "d": "M12 12h.01" }
            path { "d": "M17 12a5 5 0 00-10 0" }
            path { "d": "M19 2v2.8" }
            path { "d": "M2 5h2.8" }
            path { "d": "M22 5h-2.8" }
            path { "d": "M5 2v2.8" }
            circle { "cx": "12", "cy": "12", "r": "10" }
        }
    }
}
