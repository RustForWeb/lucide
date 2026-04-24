use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct RadioOffProps {
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
pub fn RadioOff(props: RadioOffProps) -> Element {
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
            path { "d": "M10.4103 10.7852C10.1529 11.1218 10 11.5425 10 11.999C10 13.1036 10.8954 13.999 12 13.999C12.5077 13.999 12.9713 13.8098 13.324 13.498" }
            path { "d": "M16.1992 7.80078C17.4739 9.07549 18.0422 10.8109 17.9039 12.5134" }
            path { "d": "M19.0996 4.89844C22.0892 7.88804 22.7871 12.2879 21.1932 15.936" }
            path { "d": "M2 2L22 22" }
            path { "d": "M4.89961 19.0984C0.999609 15.1984 0.999609 8.79844 4.89961 4.89844" }
            path { "d": "M7.79922 16.1992C5.66828 14.0683 5.51165 10.6498 7.32931 8.25" }
        }
    }
}
