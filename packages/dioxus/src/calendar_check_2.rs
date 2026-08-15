use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct CalendarCheck2Props {
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
pub fn CalendarCheck2(props: CalendarCheck2Props) -> Element {
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
            path { "d": "M 19 3 L 5 3" }
            path { "d": "M 21 13 L 21 5" }
            path { "d": "M 21 5 A2 2 0 0 0 19 3" }
            path { "d": "M 3 19 A2 2 0 0 0 5 21" }
            path { "d": "M 3 5 L 3 19" }
            path { "d": "M 5 3 A2 2 0 0 0 3 5" }
            path { "d": "m16 19 2 2 4-4" }
            path { "d": "M16 2v3" }
            path { "d": "M3 9h18" }
            path { "d": "M5 21 L12.5 21" }
            path { "d": "M8 2v3" }
        }
    }
}
