use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct GitPullRequestClosedProps {
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
pub fn GitPullRequestClosed(props: GitPullRequestClosedProps) -> Element {
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
            path { "d": "m15.5 3.5 5 5" }
            path { "d": "m15.5 8.5 5-5" }
            path { "d": "M18 11.62V15" }
            path { "d": "M6 9v12" }
            circle { "cx": "18", "cy": "18", "r": "3" }
            circle { "cx": "6", "cy": "6", "r": "3" }
        }
    }
}
