use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct DatabaseArrowDownProps {
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
pub fn DatabaseArrowDown(props: DatabaseArrowDownProps) -> Element {
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
            path { "d": "m16 19 3 3 3-3" }
            path { "d": "M19 16v6" }
            path { "d": "M21 12.536V5" }
            path { "d": "M3 12A9 3 0 0 0 15.182 14.806" }
            path { "d": "M3 5V19A9 3 0 0 0 13.318 21.968" }
            ellipse {
                "cx": "12",
                "cy": "5",
                "rx": "9",
                "ry": "3",
            }
        }
    }
}
