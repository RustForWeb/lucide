use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct BrainProps {
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
pub fn Brain(props: BrainProps) -> Element {
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
            path { "d": "M12 18V5" }
            path { "d": "M15 13a4.17 4.17 0 01-3-4 4.17 4.17 0 01-3 4" }
            path { "d": "M17.707 6.294A3 3 0 1012 5a3 3 0 10-5.707 1.294" }
            path { "d": "M17.996 5.125a4 4 0 012.526 5.77" }
            path { "d": "M18 18a4 4 0 002-7.465" }
            path { "d": "M19.967 17.482a4 4 0 11-7.967.517 4 4 0 11-7.967-.517" }
            path { "d": "M6 18a4 4 0 01-2-7.465" }
            path { "d": "M6.004 5.125a4 4 0 00-2.526 5.77" }
        }
    }
}
