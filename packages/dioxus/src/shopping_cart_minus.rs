use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct ShoppingCartMinusProps {
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
pub fn ShoppingCartMinus(props: ShoppingCartMinusProps) -> Element {
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
            path { "d": "M16 5h6" }
            path { "d": "m2.05 2.05 1.099-.028a1 1 0 011.008.815l2.69 14.347A1 1 0 007.83 18H18" }
            path { "d": "M4.564 5H12" }
            path { "d": "M6.25 14h12.712a2 2 0 001.991-1.57l.514-3.113" }
            circle { "cx": "18", "cy": "20", "r": "2" }
            circle { "cx": "8", "cy": "20", "r": "2" }
        }
    }
}
