use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct BugPlayProps {
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
pub fn BugPlay(props: BugPlayProps) -> Element {
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
            path { "d": "M12.765 21.522a.5.5 0 0 1-.765-.424v-8.196a.5.5 0 0 1 .765-.424l5.878 3.674a1 1 0 0 1 0 1.696z" }
            path { "d": "M14.12 3.88 16 2" }
            path { "d": "M18 11a4 4 0 0 0-4-4h-4a4 4 0 0 0-4 4v3a6.1 6.1 0 0 0 2 4.5" }
            path { "d": "M20.97 5c0 2.1-1.6 3.8-3.5 4" }
            path { "d": "M3 21c0-2.1 1.7-3.9 3.8-4" }
            path { "d": "M6 13H2" }
            path { "d": "M6.53 9C4.6 8.8 3 7.1 3 5" }
            path { "d": "m8 2 1.88 1.88" }
            path { "d": "M9 7.13v-1a3.003 3.003 0 1 1 6 0v1" }
        }
    }
}
