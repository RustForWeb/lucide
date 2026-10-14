use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct GraphStarProps {
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
pub fn GraphStar(props: GraphStarProps) -> Element {
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
            path { "d": "m12.342 9.316 1.764-3.527" }
            path { "d": "m12.59 14.545 2.35 3.759" }
            path { "d": "m13.982 12.332 4.03.447" }
            path { "d": "M5.414 17.586 8.88 14.12" }
            path { "d": "M9.025 9.742 5.317 5.505" }
            circle { "cx": "11", "cy": "12", "r": "3" }
            circle { "cx": "15", "cy": "4", "r": "2" }
            circle { "cx": "16", "cy": "20", "r": "2" }
            circle { "cx": "20", "cy": "13", "r": "2" }
            circle { "cx": "4", "cy": "19", "r": "2" }
            circle { "cx": "4", "cy": "4", "r": "2" }
        }
    }
}
