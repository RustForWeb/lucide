use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct SunriseProps {
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
pub fn Sunrise(props: SunriseProps) -> Element {
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
            path { "d": "M12 2v8" }
            path { "d": "m4.93 10.93 1.41 1.41" }
            path { "d": "M2 18h2" }
            path { "d": "M20 18h2" }
            path { "d": "m19.07 10.93-1.41 1.41" }
            path { "d": "M22 22H2" }
            path { "d": "m8 6 4-4 4 4" }
            path { "d": "M16 18a4 4 0 0 0-8 0" }
        }
    }
}
