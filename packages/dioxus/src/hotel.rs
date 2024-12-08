use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct HotelProps {
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
}
#[component]
pub fn Hotel(props: HotelProps) -> Element {
    let stroke_width = if props.absolute_stroke_width {
        props.stroke_width * 24 / props.size
    } else {
        props.stroke_width
    };
    rsx! {
        svg {
            "xmlns": "http://www.w3.org/2000/svg",
            "class": if let Some(class) = props.class { "{class}" },
            "width": "{props.size}",
            "height": "{props.size}",
            "viewBox": "0 0 24 24",
            "fill": "{props.fill}",
            "stroke": "{props.color}",
            "stroke-width": "{stroke_width}",
            "stroke-linecap": "round",
            "stroke-linejoin": "round",
            path { "d": "M10 22v-6.57" }
            path { "d": "M12 11h.01" }
            path { "d": "M12 7h.01" }
            path { "d": "M14 15.43V22" }
            path { "d": "M15 16a5 5 0 0 0-6 0" }
            path { "d": "M16 11h.01" }
            path { "d": "M16 7h.01" }
            path { "d": "M8 11h.01" }
            path { "d": "M8 7h.01" }
            rect {
                "x": "4",
                "y": "2",
                "width": "16",
                "height": "20",
                "rx": "2",
            }
        }
    }
}
