use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct LighthouseProps {
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
pub fn Lighthouse(props: LighthouseProps) -> Element {
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
            path { "d": "M12 3V2" }
            path { "d": "M16.066 16.865 7 22l2-11V6a3 3 0 016 0v5l2 11" }
            path { "d": "m19.792 4.5.866-.5" }
            path { "d": "m19.797 13.5.866.5" }
            path { "d": "M21 9h1" }
            path { "d": "M3 9H2" }
            path { "d": "m4.203 13.5-.866.5" }
            path { "d": "M4.208 4.5 3.342 4" }
            path { "d": "M5.5 22h13" }
            path { "d": "m7.932 16.875 7.377-4.178" }
            path { "d": "M8 11h8" }
            path { "d": "M8 7h8" }
        }
    }
}
