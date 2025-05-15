use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct CogProps {
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
pub fn Cog(props: CogProps) -> Element {
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
            path { "d": "M12 20a8 8 0 1 0 0-16 8 8 0 0 0 0 16Z" }
            path { "d": "M12 14a2 2 0 1 0 0-4 2 2 0 0 0 0 4Z" }
            path { "d": "M12 2v2" }
            path { "d": "M12 22v-2" }
            path { "d": "m17 20.66-1-1.73" }
            path { "d": "M11 10.27 7 3.34" }
            path { "d": "m20.66 17-1.73-1" }
            path { "d": "m3.34 7 1.73 1" }
            path { "d": "M14 12h8" }
            path { "d": "M2 12h2" }
            path { "d": "m20.66 7-1.73 1" }
            path { "d": "m3.34 17 1.73-1" }
            path { "d": "m17 3.34-1 1.73" }
            path { "d": "m11 13.73-4 6.93" }
        }
    }
}
