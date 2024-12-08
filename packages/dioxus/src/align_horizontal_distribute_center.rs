use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct AlignHorizontalDistributeCenterProps {
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
pub fn AlignHorizontalDistributeCenter(props: AlignHorizontalDistributeCenterProps) -> Element {
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
            rect {
                "width": "6",
                "height": "14",
                "x": "4",
                "y": "5",
                "rx": "2",
            }
            rect {
                "width": "6",
                "height": "10",
                "x": "14",
                "y": "7",
                "rx": "2",
            }
            path { "d": "M17 22v-5" }
            path { "d": "M17 7V2" }
            path { "d": "M7 22v-3" }
            path { "d": "M7 5V2" }
        }
    }
}
