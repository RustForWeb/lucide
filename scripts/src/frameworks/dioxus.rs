use std::{error::Error, path::PathBuf, process::Command};

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use roxmltree::{Attribute, Children, Document, Node, NodeType};

use crate::framework::Framework;
pub struct Dioxus;

impl Framework for Dioxus {
    fn name(&self) -> &'static str {
        "dioxus"
    }

    fn lib_header(&self) -> Option<String> {
        Some(
            "\
            //! Dioxus port of [Lucide](https://lucide.dev/).\n\
            //!\n\
            //! Lucide is a beautiful & consistent icon toolkit made by the community.\n\
            //!\n\
            //! See [the Rust Lucide book](https://lucide.rustforweb.org/dioxus.html) for more documenation.\n\
            "
            .to_owned()
        )
    }

    fn generate(&self, component_name: String, svg: String) -> Result<TokenStream, Box<dyn Error>> {
        let document = Document::parse(&svg)?;
        let svg = convert_children(document.root().children(), None);

        let component_name: TokenStream = component_name.parse()?;
        let props_name: TokenStream = format!("{}Props", component_name).parse()?;

        Ok(quote! {
            use dioxus::prelude::*;

            #[derive(Clone, PartialEq, Props)]
            pub struct #props_name {
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
            pub fn #component_name(props: #props_name) -> Element {
                let stroke_width = if props.absolute_stroke_width {
                    props.stroke_width * 24 / props.size
                } else {
                    props.stroke_width
                };

                rsx! {
                    #svg
                }
            }
        })
    }

    fn generate_example(&self, component_names: &[String]) -> Result<TokenStream, Box<dyn Error>> {
        let mut letter_component_name: Vec<TokenStream> = vec![];
        let mut letter_component: Vec<TokenStream> = vec![];

        for letter in 'A'..='Z' {
            let mut component_name: Vec<TokenStream> = vec![];
            let mut human_name: Vec<TokenStream> = vec![];

            for name in component_names {
                if !name.starts_with(letter) {
                    continue;
                }

                component_name.push(name.parse()?);
                human_name.push(
                    name.trim_end_matches("Icon")
                        .to_case(Case::Title)
                        .to_token_stream(),
                );
            }

            for (n, chunk) in component_name
                .into_iter()
                .zip(human_name)
                .collect::<Vec<_>>()
                .chunks(100)
                .enumerate()
            {
                let (component_name, human_name): (Vec<_>, Vec<_>) = chunk.iter().cloned().unzip();

                let name: TokenStream = format!("Icons{letter}{}", n + 1).parse()?;
                letter_component_name.push(quote! {
                    #name {}
                });

                letter_component.push(quote! {
                    #[component]
                    pub fn #name() -> Element {
                        let icons = [
                            #((rsx! { #component_name {} }, #human_name),)*
                        ];

                        rsx! {
                            for (icon, name) in icons {
                                div {
                                    key: "{name}",
                                    class: "flex flex-wrap items-center gap-4 text-sm",
                                    {icon}
                                    span {
                                        {name}
                                    }
                                }
                            }
                        }
                    }
                });
            }
        }

        Ok(quote! {
            use dioxus::prelude::*;
            use lucide_dioxus::*;

            #[component]
            pub fn Icons() -> Element {
                rsx! {
                    div {
                        class: "w-full max-w-80 py-4"
                        #(#letter_component_name)*
                    }
                }
            }

            #(#letter_component)*
        })
    }

    fn format(&self, package: String, path: PathBuf) -> Result<(), Box<dyn Error>> {
        Command::new("dx")
            .arg("fmt")
            .current_dir(path)
            .status()?
            .exit_ok()?;

        Command::new("cargo")
            .arg("fmt")
            .arg("-p")
            .arg(&package)
            .status()?
            .exit_ok()?;

        Ok(())
    }
}

fn convert_children(children: Children, parent_namespace: Option<&str>) -> TokenStream {
    children
        .into_iter()
        .filter_map(|node| convert_node(node, parent_namespace))
        .collect()
}

fn convert_node(node: Node, parent_namespace: Option<&str>) -> Option<TokenStream> {
    match node.node_type() {
        NodeType::Element => {
            let name = node
                .tag_name()
                .name()
                .parse::<TokenStream>()
                .expect("Node name should be parsed.");
            let namespace = node.tag_name().namespace().and_then(|namespace| {
                (Some(namespace) != parent_namespace).then(|| {
                    quote! {
                        "xmlns": #namespace,
                    }
                })
            });
            let class = (node.tag_name().name() == "svg").then(|| {
                quote! {
                    "class": if let Some(class) = props.class { "{class}" },
                }
            });
            let attributes = node
                .attributes()
                .map(|attribute| convert_attribute(attribute, node.tag_name().name()))
                .collect::<Vec<_>>();
            let children = convert_children(
                node.children(),
                node.tag_name().namespace().or(parent_namespace),
            );

            Some(quote! {
                #name {
                    #namespace
                    #class
                    #(#attributes,)*
                    #children
                }
            })
        }
        _ => None,
    }
}

fn convert_attribute(attribute: Attribute, node_name: &str) -> TokenStream {
    let name = attribute.name();
    let value = if node_name == "svg" {
        match name {
            "width" => "{props.size}",
            "height" => "{props.size}",
            "fill" => "{props.fill}",
            "stroke" => "{props.color}",
            "stroke-width" => "{stroke_width}",
            _ => attribute.value(),
        }
    } else {
        attribute.value()
    };

    quote! {
        #name: #value
    }
}
