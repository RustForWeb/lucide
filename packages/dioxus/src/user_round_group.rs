use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct UserRoundGroupProps {
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
pub fn UserRoundGroup(props: UserRoundGroupProps) -> Element {
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
            path { "d": "M17 21a5 5 0 00-10 0" }
            path { "d": "M22 10.5a3.5 3.5 0 00-5.507-2.868" }
            path { "d": "M7.507 7.632A3.5 3.5 0 002 10.5" }
            circle { "cx": "12", "cy": "13", "r": "3" }
            circle { "cx": "18.5", "cy": "4.5", "r": "2.5" }
            circle { "cx": "5.5", "cy": "4.5", "r": "2.5" }
        }
    }
}
