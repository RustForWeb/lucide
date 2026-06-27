use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct WebcamOffProps {
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
pub fn WebcamOff(props: WebcamOffProps) -> Element {
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
            path { "d": "M12 22v-4" }
            path { "d": "M12.754 7.096a3 3 0 0 1 2.15 2.15" }
            path { "d": "M12.863 12.873a3 3 0 0 1-3.736-3.735" }
            path { "d": "M16.566 16.57A8 8 0 0 1 5.43 5.433" }
            path { "d": "m2 2 20 20" }
            path { "d": "M7 22h10" }
            path { "d": "M8.478 2.817a8 8 0 0 1 10.705 10.705" }
        }
    }
}
