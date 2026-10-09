use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct HikingStickProps {
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
pub fn HikingStick(props: HikingStickProps) -> Element {
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
            path { "d": "M13.5 10.5 2 22" }
            path { "d": "M16.352 11.648a1.205 1.205 0 01-1.704 0l-2.296-2.296a1.205 1.205 0 010-1.704l.72-.72a2 2 0 011.022-.546l.599-.12a2 2 0 001.569-1.57l.12-.598a2 2 0 01.546-1.022l.72-.72a1.205 1.205 0 011.704 0l2.296 2.296a1.205 1.205 0 010 1.704l-6.02 6.02a1 1 0 103 3l.201-.201A7.4 7.4 0 0021 9.93V7" }
            path { "d": "m6 21-3-3" }
        }
    }
}
