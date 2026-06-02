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
            path { "d": "M12 2v2" }
            path { "d": "M12 8a4 4 0 0 0-1.645 7.647" }
            path { "d": "M2 12h2" }
            path { "d": "M20 14.54a4 4 0 1 1-4 0V4a2 2 0 0 1 4 0z" }
            path { "d": "m4.93 4.93 1.41 1.41" }
            path { "d": "m6.34 17.66-1.41 1.41" }
        }
    }
}
