use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct BrainProps {
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
pub fn Brain(props: BrainProps) -> Element {
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
            path { "d": "M12 5a3 3 0 1 0-5.997.125 4 4 0 0 0-2.526 5.77 4 4 0 0 0 .556 6.588A4 4 0 1 0 12 18Z" }
            path { "d": "M12 5a3 3 0 1 1 5.997.125 4 4 0 0 1 2.526 5.77 4 4 0 0 1-.556 6.588A4 4 0 1 1 12 18Z" }
            path { "d": "M15 13a4.5 4.5 0 0 1-3-4 4.5 4.5 0 0 1-3 4" }
            path { "d": "M17.599 6.5a3 3 0 0 0 .399-1.375" }
            path { "d": "M6.003 5.125A3 3 0 0 0 6.401 6.5" }
            path { "d": "M3.477 10.896a4 4 0 0 1 .585-.396" }
            path { "d": "M19.938 10.5a4 4 0 0 1 .585.396" }
            path { "d": "M6 18a4 4 0 0 1-1.967-.516" }
            path { "d": "M19.967 17.484A4 4 0 0 1 18 18" }
        }
    }
}
