use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct ThermometerSunProps {
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
pub fn ThermometerSun(props: ThermometerSunProps) -> Element {
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
            path { "d": "M12 9a4 4 0 0 0-2 7.5" }
            path { "d": "M12 3v2" }
            path { "d": "m6.6 18.4-1.4 1.4" }
            path { "d": "M20 4v10.54a4 4 0 1 1-4 0V4a2 2 0 0 1 4 0Z" }
            path { "d": "M4 13H2" }
            path { "d": "M6.34 7.34 4.93 5.93" }
        }
    }
}
