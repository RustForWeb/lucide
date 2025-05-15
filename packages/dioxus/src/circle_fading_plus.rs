use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct CircleFadingPlusProps {
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
pub fn CircleFadingPlus(props: CircleFadingPlusProps) -> Element {
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
            path { "d": "M12 2a10 10 0 0 1 7.38 16.75" }
            path { "d": "M12 8v8" }
            path { "d": "M16 12H8" }
            path { "d": "M2.5 8.875a10 10 0 0 0-.5 3" }
            path { "d": "M2.83 16a10 10 0 0 0 2.43 3.4" }
            path { "d": "M4.636 5.235a10 10 0 0 1 .891-.857" }
            path { "d": "M8.644 21.42a10 10 0 0 0 7.631-.38" }
        }
    }
}
