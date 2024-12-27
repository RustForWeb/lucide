use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct SnowflakeProps {
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
pub fn Snowflake(props: SnowflakeProps) -> Element {
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
            path { "d": "m10 20-1.25-2.5L6 18" }
            path { "d": "M10 4 8.75 6.5 6 6" }
            path { "d": "m14 20 1.25-2.5L18 18" }
            path { "d": "m14 4 1.25 2.5L18 6" }
            path { "d": "m17 21-3-6h-4" }
            path { "d": "m17 3-3 6 1.5 3" }
            path { "d": "M2 12h6.5L10 9" }
            path { "d": "m20 10-1.5 2 1.5 2" }
            path { "d": "M22 12h-6.5L14 15" }
            path { "d": "m4 10 1.5 2L4 14" }
            path { "d": "m7 21 3-6-1.5-3" }
            path { "d": "m7 3 3 6h4" }
        }
    }
}
