use std::{error::Error, fs, path::Path};

use convert_case::{Case, Casing};
use git2::Repository;
use log::info;
use scripts::{
    framework::Framework,
    frameworks::{leptos::Leptos, yew::Yew},
};
use tempfile::tempdir;

const GIT_URL: &str = "https://github.com/lucide-icons/lucide.git";
const GIT_REF: &str = "0.460.1";

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let frameworks: [Box<dyn Framework>; 2] = [Box::new(Leptos), Box::new(Yew)];

    let repository_path = tempdir()?;
    let repository_icons_path = repository_path.path().join("icons");

    info!(
        "Cloning \"{}\" ref \"{}\" into \"{}\".",
        GIT_URL,
        GIT_REF,
        repository_path.path().display()
    );

    git_checkout(&repository_path)?;

    info!("Generating icons.");

    let mut modules = vec![];
    let mut component_names = vec![];

    for entry in fs::read_dir(repository_icons_path)? {
        let path = entry?.path();

        if !path.is_file() || path.extension().is_none_or(|extension| extension != "svg") {
            continue;
        }

        let file_path = path.clone();
        let file_stem = file_path
            .file_stem()
            .expect("File stem should exist.")
            .to_string_lossy();

        let file_contents = fs::read_to_string(path)?;

        let module = file_stem.to_case(Case::Snake);
        modules.push(module.clone());

        let component_name = file_stem.to_case(Case::Pascal);
        component_names.push(component_name.clone());

        info!("{} - {}", module, component_name);

        for framework in &frameworks {
            generate_icon(
                &**framework,
                module.clone(),
                component_name.clone(),
                file_contents.clone(),
            )?;
        }
    }

    for framework in &frameworks {
        generate_lib(&**framework, &modules)?;
        generate_features(&**framework, &modules)?;
        generate_example(&**framework, &component_names)?;

        framework.format(
            format!("lucide-{}", framework.name()),
            Path::new("packages").join(framework.name()).join("src"),
        )?;

        framework.format(
            format!("lucide-{}-book", framework.name()),
            Path::new("book-examples")
                .join(framework.name())
                .join("src"),
        )?;
    }

    Ok(())
}

fn git_checkout<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    let repository = Repository::clone(GIT_URL, path)?;
    let (object, reference) = repository.revparse_ext(GIT_REF)?;

    repository.checkout_tree(&object, None)?;

    match reference {
        Some(reference) => {
            repository.set_head(reference.name().expect("Reference name should exist."))?
        }
        None => repository.set_head_detached(object.id())?,
    }

    Ok(())
}

fn generate_icon(
    framework: &dyn Framework,
    module: String,
    component_name: String,
    input: String,
) -> Result<(), Box<dyn Error>> {
    let output_path = Path::new("packages")
        .join(framework.name())
        .join("src")
        .join(format!("{}.rs", module));

    let output_tokens = framework.generate(component_name, input)?;
    let output = prettyplease::unparse(&syn::parse2(output_tokens)?);

    fs::write(output_path, output)?;

    Ok(())
}

fn generate_example(
    framework: &dyn Framework,
    component_names: &[String],
) -> Result<(), Box<dyn Error>> {
    let output_path = Path::new("book-examples")
        .join(framework.name())
        .join("src")
        .join("icons.rs");

    let output_tokens = framework.generate_example(component_names)?;
    let output = prettyplease::unparse(&syn::parse2(output_tokens)?);

    fs::write(output_path, output)?;

    Ok(())
}

fn generate_lib(framework: &dyn Framework, modules: &[String]) -> Result<(), Box<dyn Error>> {
    let output_path = Path::new("packages")
        .join(framework.name())
        .join("src")
        .join("lib.rs");

    let output_modules = modules
        .iter()
        .map(|module| {
            format!(
                "#[cfg(feature = \"{}\")]\nmod {};",
                module.trim_end_matches("_icon").to_case(Case::Kebab),
                sanitize_identifier(module.as_str())
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let output_uses = modules
        .iter()
        .map(|module| {
            format!(
                "#[cfg(feature = \"{}\")]\npub use {}::*;",
                module.trim_end_matches("_icon").to_case(Case::Kebab),
                sanitize_identifier(module.as_str())
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let output = format!(
        "{}{}\n\n{}\n",
        match framework.lib_header() {
            Some(header) => format!("{}\n\n", header),
            None => "".into(),
        },
        output_modules,
        output_uses
    );

    fs::write(output_path, output)?;

    Ok(())
}

fn generate_features(framework: &dyn Framework, modules: &[String]) -> Result<(), Box<dyn Error>> {
    let output_path = Path::new("packages")
        .join(framework.name())
        .join("features.toml");

    let output_features = modules
        .iter()
        .map(|module| {
            format!(
                "{} = []",
                module.trim_end_matches("_icon").to_case(Case::Kebab)
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let output_full = modules
        .iter()
        .map(|module| {
            format!(
                "\"{}\"",
                module.trim_end_matches("_icon").to_case(Case::Kebab)
            )
        })
        .collect::<Vec<String>>()
        .join(", ");

    let output = format!(
        "[features]\ndefault = []\n{}\nfull = [{}]\n",
        output_features, output_full
    );

    fs::write(output_path, output)?;

    // TODO: Replace features in Cargo.toml instead of writing to features.toml.

    Ok(())
}

fn sanitize_identifier(identifier: &str) -> &str {
    match identifier {
        "box" => "r#box",
        "move" => "r#move",
        "type" => "r#type",
        identifier => identifier,
    }
}