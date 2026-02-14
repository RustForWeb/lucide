use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct FileCogProps {
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
pub fn FileCog(props: FileCogProps) -> Element {
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
            path { "d": "M15 8a1 1 0 0 1-1-1V2a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8z" }
            path { "d": "M20 8v12a2 2 0 0 1-2 2h-4.182" }
            path { "d": "m3.305 19.53.923-.382" }
            path { "d": "M4 10.592V4a2 2 0 0 1 2-2h8" }
            path { "d": "m4.228 16.852-.924-.383" }
            path { "d": "m5.852 15.228-.383-.923" }
            path { "d": "m5.852 20.772-.383.924" }
            path { "d": "m8.148 15.228.383-.923" }
            path { "d": "m8.53 21.696-.382-.924" }
            path { "d": "m9.773 16.852.922-.383" }
            path { "d": "m9.773 19.148.922.383" }
            circle { "cx": "7", "cy": "18", "r": "3" }
        }
    }
}
