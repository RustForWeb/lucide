use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct AlignVerticalDistributeCenterProps {
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
pub fn AlignVerticalDistributeCenter(props: AlignVerticalDistributeCenterProps) -> Element {
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
            path { "d": "M22 17h-3" }
            path { "d": "M22 7h-5" }
            path { "d": "M5 17H2" }
            path { "d": "M7 7H2" }
            rect {
                "x": "5",
                "y": "14",
                "width": "14",
                "height": "6",
                "rx": "2",
            }
            rect {
                "x": "7",
                "y": "4",
                "width": "10",
                "height": "6",
                "rx": "2",
            }
        }
    }
}