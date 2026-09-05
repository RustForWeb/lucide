use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct VirusOffProps {
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
pub fn VirusOff(props: VirusOffProps) -> Element {
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
            path { "d": "M10.01 10h.01" }
            path { "d": "M12 14.991h.01" }
            path { "d": "M12 22v-3" }
            path { "d": "M12 2v3" }
            path { "d": "M13 22h-2" }
            path { "d": "M13 2h-2" }
            path { "d": "m16.5 19.794-1-1.733" }
            path { "d": "m16.5 4.205-1 1.732" }
            path { "d": "M18.891 13.235a7 7 0 00-8.126-8.126" }
            path { "d": "m19.794 7.5-1.732 1" }
            path { "d": "M2 12h3" }
            path { "d": "M2 13v-2" }
            path { "d": "m2 2 20 20" }
            path { "d": "M22 12h-3" }
            path { "d": "M22 13v-2" }
            path { "d": "m4.206 16.5 1.732-1" }
            path { "d": "m4.206 7.5 1.732 1" }
            path { "d": "M7.05 7.05a7 7 0 009.9 9.9" }
            path { "d": "m7.5 19.794 1-1.733" }
            path { "d": "M9 12h.01" }
        }
    }
}
