use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct FolderCogProps {
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
pub fn FolderCog(props: FolderCogProps) -> Element {
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
            circle { "cx": "18", "cy": "18", "r": "3" }
            path { "d": "M10.3 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v3.3" }
            path { "d": "m21.7 19.4-.9-.3" }
            path { "d": "m15.2 16.9-.9-.3" }
            path { "d": "m16.6 21.7.3-.9" }
            path { "d": "m19.1 15.2.3-.9" }
            path { "d": "m19.6 21.7-.4-1" }
            path { "d": "m16.8 15.3-.4-1" }
            path { "d": "m14.3 19.6 1-.4" }
            path { "d": "m20.7 16.8 1-.4" }
        }
    }
}
