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
            path { "d": "M12 20v-8" }
            path { "d": "M12.656 7H14a4 4 0 0 1 4 4v1.344" }
            path { "d": "M14.12 3.88 16 2" }
            path { "d": "M17.123 17.123A6 6 0 0 1 6 14v-3a4 4 0 0 1 1.72-3.287" }
            path { "d": "m2 2 20 20" }
            path { "d": "M21 5a4 4 0 0 1-3.55 3.97" }
            path { "d": "M22 13h-3.344" }
            path { "d": "M3 21a4 4 0 0 1 3.81-4" }
            path { "d": "M3 5a4 4 0 0 0 3.55 3.97" }
            path { "d": "M6 13H2" }
            path { "d": "m8 2 1.88 1.88" }
            path { "d": "M9.712 4.06A3 3 0 0 1 15 6v1.13" }
        }
    }
}
