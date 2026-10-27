use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct HockeyStickProps {
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
pub fn HockeyStick(props: HockeyStickProps) -> Element {
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
            path { "d": "M11 4v4c0 1.1-2.025 2-4.5 2S2 9.1 2 8V4" }
            path { "d": "M18.21 3.106a2 2 0 013.579 1.789l-6.895 13.789A6 6 0 019.529 22H4.5a1 1 0 010-5h5.528a2 2 0 001.788-1.105z" }
            path { "d": "m18.59 11.295-3.578-1.789" }
            path { "d": "m20.79 6.895-3.578-1.789" }
            path { "d": "M7 17v5" }
            ellipse {
                "cx": "6.5",
                "cy": "4",
                "rx": "4.5",
                "ry": "2",
            }
        }
    }
}
