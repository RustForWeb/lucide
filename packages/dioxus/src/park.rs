use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct ParkProps {
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
pub fn Park(props: ParkProps) -> Element {
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
            path { "d": "M12 18h10" }
            path { "d": "M13.248 9.998A4.5 4.5 0 0 0 11.75 8.6V8a1 1 0 0 0-7.5 0 4.9 4.9 0 0 0 2.25 9H8" }
            path { "d": "m15 14-2 6" }
            path { "d": "m19 14 2 6" }
            path { "d": "M21 14h-8" }
            path { "d": "M8 20v-5.922a2 2 0 0 0-.586-1.414L6.5 11.75" }
            path { "d": "M9.205 12.795 8 14" }
            circle { "cx": "19", "cy": "6", "r": "2" }
        }
    }
}
