use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct ShieldCogCornerProps {
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
pub fn ShieldCogCorner(props: ShieldCogCornerProps) -> Element {
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
            path { "d": "M11 22c-3.806-1.45-7-3.966-7-9V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1v4" }
            path { "d": "M14.923 16.547 14 16.164" }
            path { "d": "m14.923 18.843-.923.383" }
            path { "d": "M16.547 14.923 16.164 14" }
            path { "d": "m16.547 20.467-.383.924" }
            path { "d": "m18.843 14.923.383-.923" }
            path { "d": "m19.225 21.391-.382-.924" }
            path { "d": "m20.467 16.547.923-.383" }
            path { "d": "m20.467 18.843.923.383" }
            circle { "cx": "17.695", "cy": "17.695", "r": "3" }
        }
    }
}
