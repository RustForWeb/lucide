use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct UsersCollaborationProps {
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
pub fn UsersCollaboration(props: UsersCollaborationProps) -> Element {
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
            path { "d": "M12.925 3.047a9 9 0 017.616 6.11" }
            path { "d": "M13 22a2 2 0 012-2h4a2 2 0 012 2" }
            path { "d": "M3 12a2 2 0 012-2h4a2 2 0 012 2" }
            path { "d": "M3.889 15.9a9 9 0 005.247 4.632" }
            circle { "cx": "17", "cy": "14", "r": "2" }
            circle { "cx": "7", "cy": "4", "r": "2" }
        }
    }
}
