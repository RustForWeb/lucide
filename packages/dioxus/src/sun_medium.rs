use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct SunMediumProps {
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
pub fn SunMedium(props: SunMediumProps) -> Element {
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
            circle { "cx": "12", "cy": "12", "r": "4" }
            path { "d": "M12 3v1" }
            path { "d": "M12 20v1" }
            path { "d": "M3 12h1" }
            path { "d": "M20 12h1" }
            path { "d": "m18.364 5.636-.707.707" }
            path { "d": "m6.343 17.657-.707.707" }
            path { "d": "m5.636 5.636.707.707" }
            path { "d": "m17.657 17.657.707.707" }
        }
    }
}
