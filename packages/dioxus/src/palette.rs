use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct PaletteProps {
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
pub fn Palette(props: PaletteProps) -> Element {
    let stroke_width = if props.absolute_stroke_width {
        props.stroke_width * 24 / props.size
    } else {
        props.stroke_width
    };
    rsx! {
        svg {
            "xmlns": "http://www.w3.org/2000/svg",
            "class": if let Some(class) = props.class { "{class}" },
            "style": if let Some(style) = props.style { "{style}" },
            "width": "{props.size}",
            "height": "{props.size}",
            "viewBox": "0 0 24 24",
            "fill": "{props.fill}",
            "stroke": "{props.color}",
            "stroke-width": "{stroke_width}",
            "stroke-linecap": "round",
            "stroke-linejoin": "round",
            path { "d": "M12 22a1 1 0 0 1 0-20 10 9 0 0 1 10 9 5 5 0 0 1-5 5h-2.25a1.75 1.75 0 0 0-1.4 2.8l.3.4a1.75 1.75 0 0 1-1.4 2.8z" }
            circle {
                "cx": "13.5",
                "cy": "6.5",
                "r": ".5",
                "fill": "currentColor",
            }
            circle {
                "cx": "17.5",
                "cy": "10.5",
                "r": ".5",
                "fill": "currentColor",
            }
            circle {
                "cx": "6.5",
                "cy": "12.5",
                "r": ".5",
                "fill": "currentColor",
            }
            circle {
                "cx": "8.5",
                "cy": "7.5",
                "r": ".5",
                "fill": "currentColor",
            }
        }
    }
}
