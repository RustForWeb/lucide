use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct SquircleDashedProps {
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
pub fn SquircleDashed(props: SquircleDashedProps) -> Element {
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
            path { "d": "M13.77 3.043a34 34 0 0 0-3.54 0" }
            path { "d": "M13.771 20.956a33 33 0 0 1-3.541.001" }
            path { "d": "M20.18 17.74c-.51 1.15-1.29 1.93-2.439 2.44" }
            path { "d": "M20.18 6.259c-.51-1.148-1.291-1.929-2.44-2.438" }
            path { "d": "M20.957 10.23a33 33 0 0 1 0 3.54" }
            path { "d": "M3.043 10.23a34 34 0 0 0 .001 3.541" }
            path { "d": "M6.26 20.179c-1.15-.508-1.93-1.29-2.44-2.438" }
            path { "d": "M6.26 3.82c-1.149.51-1.93 1.291-2.44 2.44" }
        }
    }
}
