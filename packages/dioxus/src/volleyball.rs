use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct VolleyballProps {
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
pub fn Volleyball(props: VolleyballProps) -> Element {
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
            path { "d": "M11 7a16 16 20 0 1 10.98 4.362" }
            path { "d": "M12 12a13 13 0 0 1-8.66 5" }
            path { "d": "M16.83 13.634a16 16 0 0 1-9.267 7.328" }
            path { "d": "M20.66 17A13 13 0 0 0 12 12a13 13 0 0 1 0-10" }
            path { "d": "M8.17 15.366a16 16 0 0 1-1.713-11.69" }
            circle { "cx": "12", "cy": "12", "r": "10" }
        }
    }
}
