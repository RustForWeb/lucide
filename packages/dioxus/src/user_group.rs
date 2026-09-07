use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct UserGroupProps {
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
pub fn UserGroup(props: UserGroupProps) -> Element {
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
            path { "d": "M17 21v-1a2 2 0 00-2-2H9a2 2 0 00-2 2v1" }
            path { "d": "M19 10h1a2 2 0 012 2v1" }
            path { "d": "M5 10H4a2 2 0 00-2 2v1" }
            circle { "cx": "12", "cy": "11", "r": "3" }
            circle { "cx": "18", "cy": "4", "r": "2" }
            circle { "cx": "6", "cy": "4", "r": "2" }
        }
    }
}
