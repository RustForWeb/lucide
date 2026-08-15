use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct LayerArrowDownProps {
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
pub fn LayerArrowDown(props: LayerArrowDownProps) -> Element {
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
            path { "d": "M12 10v10" }
            path { "d": "M22 10a1 1 0 01-.59.92l-5.077 2.308" }
            path { "d": "M22.017 10.005a1 1 0 00-.597-.916l-8.59-3.91a2 2 0 00-1.66.001L2.6 9.08a1 1 0 00-.02 1.831l5.093 2.316" }
            path { "d": "m9 17 3 3 3-3" }
        }
    }
}
