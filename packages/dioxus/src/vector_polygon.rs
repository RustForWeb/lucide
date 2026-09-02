use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct VectorPolygonProps {
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
pub fn VectorPolygon(props: VectorPolygonProps) -> Element {
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
            path { "d": "m12.828 4.813 5.344 2.375" }
            path { "d": "m15.769 18.153 3.461-8.306" }
            path { "d": "m5.687 14.074 7.625 4.852" }
            path { "d": "M9.772 5.579 5.228 11.42" }
            circle { "cx": "11", "cy": "4", "r": "2" }
            circle { "cx": "15", "cy": "20", "r": "2" }
            circle { "cx": "20", "cy": "8", "r": "2" }
            circle { "cx": "4", "cy": "13", "r": "2" }
        }
    }
}
