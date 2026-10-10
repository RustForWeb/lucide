use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct ScrewProps {
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
pub fn Screw(props: ScrewProps) -> Element {
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
            path { "d": "M10 9v10" }
            path { "d": "M14 13v2" }
            path { "d": "M16.5 10.5 5.706 21.294A2.4 2.4 0 014.002 22H2.5a.5.5 0 01-.5-.5v-1.502a2.4 2.4 0 01.706-1.704L13.5 7.5" }
            path { "d": "M20.819 11.575a6 6 0 00.969-5.162.6.6 0 00-.981-.219l-.455.454a1.2 1.2 0 01-1.704 0l-1.296-1.296a1.2 1.2 0 010-1.704l.454-.453a.595.595 0 00-.219-.98 6 6 0 00-5.162.967 2 2 0 00-.222 3.019l5.596 5.597a2 2 0 003.02-.222" }
            path { "d": "M6 13v8" }
        }
    }
}
