use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct ChessQueenProps {
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
pub fn ChessQueen(props: ChessQueenProps) -> Element {
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
            path { "d": "m10.68 5.499-.996 4.982a.65.65 0 01-1.131.295L5.885 7.663" }
            path { "d": "m13.32 5.5.995 4.98a.65.65 0 001.131.296l2.668-3.113" }
            path { "d": "M20.428 8.952 17.5 18" }
            path { "d": "M4 20a2 2 0 012-2h12a2 2 0 012 2v1a1 1 0 01-1 1H5a1 1 0 01-1-1z" }
            path { "d": "M6.5 18 3.571 8.952" }
            circle { "cx": "12", "cy": "4", "r": "2" }
            circle { "cx": "20", "cy": "7", "r": "2" }
            circle { "cx": "4", "cy": "7", "r": "2" }
        }
    }
}
