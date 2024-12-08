use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct BotProps {
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
pub fn Bot(props: BotProps) -> Element {
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
            path { "d": "M12 8V4H8" }
            rect {
                "width": "16",
                "height": "12",
                "x": "4",
                "y": "8",
                "rx": "2",
            }
            path { "d": "M2 14h2" }
            path { "d": "M20 14h2" }
            path { "d": "M15 13v2" }
            path { "d": "M9 13v2" }
        }
    }
}