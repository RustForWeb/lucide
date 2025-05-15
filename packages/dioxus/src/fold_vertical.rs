use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct FoldVerticalProps {
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
pub fn FoldVertical(props: FoldVerticalProps) -> Element {
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
            path { "d": "M12 22v-6" }
            path { "d": "M12 8V2" }
            path { "d": "M4 12H2" }
            path { "d": "M10 12H8" }
            path { "d": "M16 12h-2" }
            path { "d": "M22 12h-2" }
            path { "d": "m15 19-3-3-3 3" }
            path { "d": "m15 5-3 3-3-3" }
        }
    }
}
