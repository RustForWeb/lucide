use std::{collections::HashSet, error::Error, fs, path::Path};

use convert_case::{Case, Casing};
use git2::Repository;
use log::info;
use scripts::{
    framework::Framework,
    frameworks::{dioxus::Dioxus, leptos::Leptos, yew::Yew},
    metadata::Metadata,
};
use tempfile::tempdir;

const GIT_URL: &str = "https://github.com/lucide-icons/lucide.git";
const GIT_REF: &str = "0.507.0";

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let frameworks: [Box<dyn Framework>; 3] = [Box::new(Dioxus), Box::new(Leptos), Box::new(Yew)];

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

    let mut paths = vec![];
    for entry in fs::read_dir(repository_icons_path)? {
        let path = entry?.path();

        if !path.is_file() || path.extension().is_none_or(|extension| extension != "svg") {
            continue;
        }

        let file_path = path.clone();
        let file_stem = file_path
            .file_stem()
            .expect("File stem should exist.")
            .to_string_lossy()
            .to_string();

        paths.push((file_path, file_stem));
    }

    paths.sort_by_key(|(_, file_stem)| file_stem.clone());

    let mut modules = vec![];
    let mut component_names = vec![];
    let mut metadatas = vec![];

    for (path, file_stem) in paths {
        let file_contents = fs::read_to_string(&path)?;

        let mut metadata_path = path.clone();
        metadata_path.set_extension("json");
        let metadata = serde_json::from_str::<Metadata>(&fs::read_to_string(&metadata_path)?)?;
        metadatas.push(metadata.clone());

        let module = file_stem.to_case(Case::Snake);
        modules.push(module.clone());

        let component_name = file_stem.to_case(Case::Pascal);
        component_names.push(component_name.clone());

        info!("{module} - {component_name}");

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
        generate_lib(&**framework, &modules, &metadatas)?;
        generate_example(&**framework, &component_names)?;
        generate_features(&**framework, &metadatas)?;

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
        .join(format!("{module}.rs"));

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

fn generate_lib(
    framework: &dyn Framework,
    modules: &[String],
    metadatas: &[Metadata],
) -> Result<(), Box<dyn Error>> {
    let attributes = metadatas
        .iter()
        .map(|metadata| {
            let conditions = metadata
                .categories
                .iter()
                .map(|category| format!("feature = \"{category}\""))
                .collect::<Vec<_>>();

            if conditions.len() == 1 {
                format!("#[cfg({})]", conditions.join(", "))
            } else {
                format!("#[cfg(any({}))]", conditions.join(", "))
            }
        })
        .collect::<Vec<String>>();

    let output_path = Path::new("packages")
        .join(framework.name())
        .join("src")
        .join("lib.rs");

    let output_modules = modules
        .iter()
        .zip(&attributes)
        .map(|(module, attribute)| {
            format!("{attribute}\nmod {};", sanitize_identifier(module.as_str()))
        })
        .collect::<Vec<String>>()
        .join("\n");

    let output_uses = modules
        .iter()
        .zip(&attributes)
        .map(|(module, attribute)| {
            format!(
                "{attribute}\npub use {}::*;",
                sanitize_identifier(module.as_str())
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let output = format!(
        "{}{}\n\n{}\n",
        match framework.lib_header() {
            Some(header) => format!("{header}\n\n"),
            None => "".into(),
        },
        output_modules,
        output_uses
    );

    fs::write(output_path, output)?;

    Ok(())
}

fn generate_features(
    framework: &dyn Framework,
    metadatas: &[Metadata],
) -> Result<(), Box<dyn Error>> {
    let file_path = Path::new("packages")
        .join(framework.name())
        .join("Cargo.toml");

    let mut file_contents = fs::read_to_string(&file_path)?;

    let index = file_contents.find("[features]");
    if let Some(index) = index {
        file_contents = file_contents[0..index].to_string();
    }
    file_contents = file_contents.trim_end_matches("\n").to_string();

    let mut features = metadatas
        .iter()
        .flat_map(|metadata| metadata.categories.clone())
        .collect::<HashSet<String>>()
        .into_iter()
        .collect::<Vec<String>>();
    features.sort();

    let output_all_features = features
        .iter()
        .map(|feature| format!("    \"{feature}\","))
        .collect::<Vec<String>>()
        .join("\n");
    let output_features = features
        .into_iter()
        .map(|feature| format!("{feature} = []"))
        .collect::<Vec<String>>()
        .join("\n");

    let output = format!(
        "{file_contents}\n\n[features]\ndefault = []\n{output_features}\nall-icons = [\n{output_all_features}\n]\n"
    );

    fs::write(&file_path, output)?;

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
