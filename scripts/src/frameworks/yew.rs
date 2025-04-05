use std::{error::Error, path::PathBuf, process::Command};

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::framework::Framework;

pub struct Yew;

impl Framework for Yew {
    fn name(&self) -> &'static str {
        "yew"
    }

    fn lib_header(&self) -> Option<String> {
        Some(
            "\
            //! Yew port of [Lucide](https://lucide.dev/).\n\
            //!\n\
            //! Lucide is a beautiful & consistent icon toolkit made by the community.\n\
            //!\n\
            //! See [the Rust Lucide book](https://lucide.rustforweb.org/yew.html) for more documenation.\n\
            \n\
            #![allow(ambiguous_glob_reexports)]
            ".to_owned()
        )
    }

    fn generate(&self, component_name: String, svg: String) -> Result<TokenStream, Box<dyn Error>> {
        let component_name: TokenStream = component_name.parse()?;
        let props_name: TokenStream = format!("{}Props", component_name).parse()?;
        let svg: TokenStream = svg
            .replacen(
                "<svg",
                "<svg ref={props.node_ref.clone()} class={classes!(\"lucide\", props.class.clone())} ",
                1,
            )
            .replacen("width=\"24\"", "width={props.size.to_string()}", 1)
            .replacen("height=\"24\"", "height={props.size.to_string()}", 1)
            .replacen("fill=\"none\"", "fill={&props.fill}", 1)
            .replacen("stroke=\"currentColor\"", "stroke={&props.color}", 1)
            .replacen(
                "stroke-width=\"2\"",
                "stroke-width={stroke_width.to_string()}",
                1,
            )
            .parse()?;

        Ok(quote! {
            use yew::prelude::*;

            #[derive(PartialEq, Properties)]
            pub struct #props_name {
                #[prop_or(24)]
                pub size: usize,
                #[prop_or(AttrValue::from("currentColor"))]
                pub color: AttrValue,
                #[prop_or(AttrValue::from("none"))]
                pub fill: AttrValue,
                #[prop_or(2)]
                pub stroke_width: usize,
                #[prop_or(false)]
                pub absolute_stroke_width: bool,
                #[prop_or_default]
                pub class: Classes,
                #[prop_or_default]
                pub node_ref: NodeRef,
            }

            #[function_component]
            pub fn #component_name(props: &#props_name) -> Html {
                let stroke_width = if props.absolute_stroke_width {
                    props.stroke_width * 24 / props.size
                } else {
                    props.stroke_width
                };

                html! {
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

            let name: TokenStream = format!("Icons{letter}").parse()?;
            letter_component_name.push(quote! {
                <#name />
            });

            letter_component.push(quote! {
                #[function_component]
                pub fn #name() -> Html {
                    let icons = [
                        #((html! { <#component_name /> }, #human_name),)*
                    ];

                    icons
                        .into_iter()
                        .map(|(icon, name)| html! {
                            <div class="flex flex-wrap items-center gap-4 text-sm">
                                {icon}
                                <span>{name}</span>
                            </div>
                        })
                        .collect::<Html>()
                }
            });
        }

        Ok(quote! {
            use lucide_yew::{*, Component};
            use yew::prelude::*;

            #[function_component]
            pub fn Icons() -> Html {
                html! {
                    <div class="w-full max-w-80 py-4">
                        #(#letter_component_name)*
                    </div>
                }
            }

            #(#letter_component)*
        })
    }

    fn format(&self, package: String, _path: PathBuf) -> Result<(), Box<dyn Error>> {
        Command::new("cargo")
            .arg("fmt")
            .arg("-p")
            .arg(&package)
            .env("RUSTFMT", "yew-fmt")
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
