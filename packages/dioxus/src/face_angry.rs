use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct FaceAngryProps {
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
pub fn FaceAngry(props: FaceAngryProps) -> Element {
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
            path { "d": "M15 12v-1.584" }
            path { "d": "M17 10a5 5 0 00-3 1" }
            path { "d": "M7 10a5 5 0 013 1" }
            path { "d": "M9 12v-1.584" }
            path { "d": "M9 17a5 5 0 016.001 0" }
            circle { "cx": "12", "cy": "12", "r": "10" }
        }
    }
}
