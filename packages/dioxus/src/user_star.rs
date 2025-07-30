use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct UserStarProps {
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
pub fn UserStar(props: UserStarProps) -> Element {
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
            path { "d": "M8.5 15H7a4 4 0 0 0-4 4v2" }
            path { "d": "M16.5 12.903a.229.229 0 0 1 .41 0l.997 2.022a.92.92 0 0 0 .69.501l2.23.326a.229.229 0 0 1 .127.392l-1.613 1.57a.92.92 0 0 0-.264.812l.381 2.22a.229.229 0 0 1-.333.241L17.13 19.94a.92.92 0 0 0-.853 0l-1.993 1.048a.229.229 0 0 1-.333-.242l.38-2.22a.92.92 0 0 0-.264-.81l-1.613-1.571a.229.229 0 0 1 .127-.392l2.23-.326a.92.92 0 0 0 .69-.501z" }
            circle { "cx": "10", "cy": "7", "r": "4" }
        }
    }
}
