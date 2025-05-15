use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct BugOffProps {
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
pub fn BugOff(props: BugOffProps) -> Element {
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
            path { "d": "M15 7.13V6a3 3 0 0 0-5.14-2.1L8 2" }
            path { "d": "M14.12 3.88 16 2" }
            path { "d": "M22 13h-4v-2a4 4 0 0 0-4-4h-1.3" }
            path { "d": "M20.97 5c0 2.1-1.6 3.8-3.5 4" }
            path { "d": "m2 2 20 20" }
            path { "d": "M7.7 7.7A4 4 0 0 0 6 11v3a6 6 0 0 0 11.13 3.13" }
            path { "d": "M12 20v-8" }
            path { "d": "M6 13H2" }
            path { "d": "M3 21c0-2.1 1.7-3.9 3.8-4" }
        }
    }
}
