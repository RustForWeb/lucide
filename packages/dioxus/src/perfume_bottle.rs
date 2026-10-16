use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct PerfumeBottleProps {
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
pub fn PerfumeBottle(props: PerfumeBottleProps) -> Element {
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
            path { "d": "M 5 22 H 11.072" }
            path { "d": "M15 5 C 23 4 23.328 12.676 20 14.536" }
            path { "d": "M15.802 10.308 C 14.776 9.465 14 8.496 14 7" }
            path { "d": "M2 2h.01" }
            path { "d": "M2 7h.01" }
            path { "d": "M5 4.5h.01" }
            path { "d": "M8.127 22 A 8 8 0 0 1 4 15 3 3 0 0 1 4.879 12.879C 6.879 10.879 10 10 10 7" }
            circle { "cx": "18", "cy": "18", "r": "4" }
            rect {
                "x": "9",
                "y": "2",
                "width": "6",
                "height": "5",
                "rx": "1",
            }
        }
    }
}
