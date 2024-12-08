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
            "class": if let Some(class) = props.class { "{class}" },
            "width": "{props.size}",
            "height": "{props.size}",
            "viewBox": "0 0 24 24",
            "fill": "{props.fill}",
            "stroke": "{props.color}",
            "stroke-width": "{stroke_width}",
            "stroke-linecap": "round",
            "stroke-linejoin": "round",
            path { "d": "M12 5a3 3 0 1 0-5.997.142 4 4 0 0 0-2.526 5.77 4 4 0 0 0 .556 6.588 4 4 0 0 0 7.636 2.106 3.2 3.2 0 0 0 .164-.546c.028-.13.306-.13.335 0a3.2 3.2 0 0 0 .163.546 4 4 0 0 0 7.636-2.106 4 4 0 0 0 .556-6.588 4 4 0 0 0-2.526-5.77A3 3 0 1 0 12 5" }
            path { "d": "M17.599 6.5a3 3 0 0 0 .399-1.375" }
            path { "d": "M6.003 5.125A3 3 0 0 0 6.401 6.5" }
            path { "d": "M3.477 10.896a4 4 0 0 1 .585-.396" }
            path { "d": "M19.938 10.5a4 4 0 0 1 .585.396" }
            path { "d": "M6 18a4 4 0 0 1-1.967-.516" }
            path { "d": "M19.967 17.484A4 4 0 0 1 18 18" }
            circle { "cx": "12", "cy": "12", "r": "3" }
            path { "d": "m15.7 10.4-.9.4" }
            path { "d": "m9.2 13.2-.9.4" }
            path { "d": "m13.6 15.7-.4-.9" }
            path { "d": "m10.8 9.2-.4-.9" }
            path { "d": "m15.7 13.5-.9-.4" }
            path { "d": "m9.2 10.9-.9-.4" }
            path { "d": "m10.5 15.7.4-.9" }
            path { "d": "m13.1 9.2.4-.9" }
        }
    }
}
