use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct BrainCogProps {
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
pub fn BrainCog(props: BrainCogProps) -> Element {
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
            path { "d": "m10.852 14.772-.383.923" }
            path { "d": "m10.852 9.228-.383-.923" }
            path { "d": "m13.148 14.772.382.924" }
            path { "d": "m13.53 8.305-.382.923" }
            path { "d": "m14.772 10.852.923-.383" }
            path { "d": "m14.772 13.148.923.383" }
            path { "d": "M17.997 5.125a4 4 0 012.526 5.77" }
            path { "d": "M18 18a4 4 0 002-7.464" }
            path { "d": "M4.033 17.483A4 4 0 0011.46 20c.184-.31.896-.31 1.076 0a4 4 0 007.43-2.517" }
            path { "d": "M6 18a4 4 0 01-2-7.464" }
            path { "d": "M6.002 5.125a4 4 0 00-2.525 5.77" }
            path { "d": "M6.293 6.294a3 3 0 01-.287-1.098l-.004-.07a3 3 0 015.968-.543c.05.02.057.01.059 0a3 3 0 015.968.542l-.004.071a3 3 0 01-.287 1.098" }
            path { "d": "m9.228 10.852-.923-.383" }
            path { "d": "m9.228 13.148-.923.383" }
            circle { "cx": "12", "cy": "12", "r": "3" }
        }
    }
}
