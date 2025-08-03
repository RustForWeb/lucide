use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct MessageSquareDashedProps {
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
pub fn MessageSquareDashed(props: MessageSquareDashedProps) -> Element {
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
            path { "d": "M12 19h.01" }
            path { "d": "M12 3h.01" }
            path { "d": "M16 19h.01" }
            path { "d": "M16 3h.01" }
            path { "d": "M2 13h.01" }
            path { "d": "M2 17v4.286a.71.71 0 0 0 1.212.502l2.202-2.202A2 2 0 0 1 6.828 19H8" }
            path { "d": "M2 5a2 2 0 0 1 2-2" }
            path { "d": "M2 9h.01" }
            path { "d": "M20 3a2 2 0 0 1 2 2" }
            path { "d": "M22 13h.01" }
            path { "d": "M22 17a2 2 0 0 1-2 2" }
            path { "d": "M22 9h.01" }
            path { "d": "M8 3h.01" }
        }
    }
}
