use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct RadioTowerProps {
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
pub fn RadioTower(props: RadioTowerProps) -> Element {
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
            path { "d": "M4.9 16.1C1 12.2 1 5.8 4.9 1.9" }
            path { "d": "M7.8 4.7a6.14 6.14 0 0 0-.8 7.5" }
            circle { "cx": "12", "cy": "9", "r": "2" }
            path { "d": "M16.2 4.8c2 2 2.26 5.11.8 7.47" }
            path { "d": "M19.1 1.9a9.96 9.96 0 0 1 0 14.1" }
            path { "d": "M9.5 18h5" }
            path { "d": "m8 22 4-11 4 11" }
        }
    }
}
