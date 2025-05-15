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
                "cx": "8.5",
                "cy": "7.5",
                "r": ".5",
                "fill": "currentColor",
            }
            circle {
                "cx": "6.5",
                "cy": "12.5",
                "r": ".5",
                "fill": "currentColor",
            }
            path { "d": "M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10c.926 0 1.648-.746 1.648-1.688 0-.437-.18-.835-.437-1.125-.29-.289-.438-.652-.438-1.125a1.64 1.64 0 0 1 1.668-1.668h1.996c3.051 0 5.555-2.503 5.555-5.554C21.965 6.012 17.461 2 12 2z" }
        }
    }
}
