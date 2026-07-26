use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct RotateCwFadingClockProps {
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
pub fn RotateCwFadingClock(props: RotateCwFadingClockProps) -> Element {
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
            path { "d": "M12 3a9.75 9.75 0 0 1 6.74 2.74" }
            path { "d": "M18.74 5.74 21 8" }
            path { "d": "M21 8V3" }
            path { "d": "M7.5 19.794c-6-3.464-6-12.124 0-15.588" }
            path { "d": "M7.5 4.206A9 9 0 0 1 12 3" }
            path { "d": "M12 7v5l4 2" }
            path { "d": "M14 20.775A9 9 0 0 1 12 21" }
            path { "d": "M19 17.656a9 9 0 0 1-1.5 1.456" }
            path { "d": "M21 12a9 9 0 0 1-.228 2" }
            path { "d": "M21 8h-5" }
        }
    }
}
