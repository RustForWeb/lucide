use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct CloudCogProps {
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
pub fn CloudCog(props: CloudCogProps) -> Element {
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
            path { "d": "m10.852 19.772-.383.924" }
            path { "d": "m13.148 14.228.383-.923" }
            path { "d": "M13.148 19.772a3 3 0 1 0-2.296-5.544l-.383-.923" }
            path { "d": "m13.53 20.696-.382-.924a3 3 0 1 1-2.296-5.544" }
            path { "d": "m14.772 15.852.923-.383" }
            path { "d": "m14.772 18.148.923.383" }
            path { "d": "M4.2 15.1a7 7 0 1 1 9.93-9.858A7 7 0 0 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.2" }
            path { "d": "m9.228 15.852-.923-.383" }
            path { "d": "m9.228 18.148-.923.383" }
        }
    }
}
