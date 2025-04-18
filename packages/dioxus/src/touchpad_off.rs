use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct TouchpadOffProps {
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
pub fn TouchpadOff(props: TouchpadOffProps) -> Element {
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
            path { "d": "M4 4a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h16" }
            path { "d": "M2 14h12" }
            path { "d": "M22 14h-2" }
            path { "d": "M12 20v-6" }
            path { "d": "m2 2 20 20" }
            path { "d": "M22 16V6a2 2 0 0 0-2-2H10" }
        }
    }
}
