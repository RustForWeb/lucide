use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct CupcakeProps {
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
pub fn Cupcake(props: CupcakeProps) -> Element {
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
            path { "d": "M12 22v-9" }
            path { "d": "M14 4h1a3 3 0 013 3l-.004.125A4 4 0 0121 11v2" }
            path { "d": "m15.5 22 1.5-9" }
            path { "d": "M21 13a1 1 0 01.919 1.394l-2.74 6.394A2 2 0 0117.34 22H6.659a2 2 0 01-1.838-1.212l-2.74-6.394A1 1 0 013 13z" }
            path { "d": "M3 13v-2a4 4 0 013.003-3.875L6 7a3 3 0 013-3h1" }
            path { "d": "M8.5 22 7 13" }
            circle { "cx": "12", "cy": "4", "r": "2" }
        }
    }
}
