use std::{env, error::Error, path::Path};

use git2::{BranchType, Cred, IndexAddOption, PushOptions, RemoteCallbacks, Repository, Signature};
use log::info;
use semver::Version;

use crate::{GITHUB_OWNER, GITHUB_REPO, UPSTREAM_GIT_URL};

pub fn git_checkout<P: AsRef<Path>>(
    path: P,
    url: &str,
    r#ref: &str,
) -> Result<Repository, Box<dyn Error>> {
    info!(
        "Cloning \"{}\" ref \"{}\" into \"{}\".",
        UPSTREAM_GIT_URL,
        r#ref,
        path.as_ref().display()
    );

    let repository = Repository::clone(url, path)?;

    {
        let (object, reference) = repository.revparse_ext(r#ref)?;

        repository.checkout_tree(&object, None)?;

        match reference {
            Some(reference) => {
                repository.set_head(reference.name().expect("Reference name should exist."))?
            }
            None => repository.set_head_detached(object.id())?,
        }
    }

    Ok(repository)
}

pub fn git_version_tags(repository: &Repository) -> Result<Vec<Version>, Box<dyn Error>> {
    let tag_names = repository.tag_names(None)?;
    let mut tag_names = tag_names
        .into_iter()
        .flat_map(|tag_name| {
            tag_name.and_then(|tag_name| Version::parse(tag_name.trim_start_matches("v")).ok())
        })
        .collect::<Vec<_>>();

    tag_names.sort();

    Ok(tag_names)
}

pub fn git_has_remote_branch(repository: &Repository, branch_name: &str) -> bool {
    repository
        .find_branch(branch_name, BranchType::Remote)
        .is_ok()
}

pub fn git_create_branch(repository: &Repository, branch_name: &str) -> Result<(), Box<dyn Error>> {
    info!("Creating branch {branch_name}.");

    let branch = repository.branch(branch_name, &repository.head()?.peel_to_commit()?, false)?;

    repository.set_head(
        branch
            .into_reference()
            .name()
            .expect("Reference name should exist."),
    )?;

    Ok(())
}

pub fn git_commit(repository: &Repository, message: &str) -> Result<(), Box<dyn Error>> {
    info!("Committing.");

    repository
        .index()?
        .add_all(["."], IndexAddOption::DEFAULT, None)?;
    repository.index()?.write()?;

    let signature = Signature::now(&env::var("GIT_USER_NAME")?, &env::var("GIT_USER_EMAIL")?)?;
    let tree = repository.find_tree(repository.index()?.write_tree()?)?;
    let parent = repository.head()?.peel_to_commit()?;

    repository.commit(
        Some("HEAD"),
        &signature,
        &signature,
        message,
        &tree,
        &[&parent],
    )?;

    Ok(())
}

pub fn git_push(repository: &Repository, branch_name: &str) -> Result<(), Box<dyn Error>> {
    info!("Pushing branch {branch_name}.");

    let branch = repository.find_branch(branch_name, BranchType::Local)?;
    let branch_reference = branch.into_reference();
    let branch_ref = branch_reference
        .name()
        .expect("Reference name should exist.");

    let username = env::var("GITHUB_ACTOR")?;
    let password = env::var("GITHUB_TOKEN")?;

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_, _, _| Cred::userpass_plaintext(&username, &password));

    let mut options = PushOptions::new();
    options.remote_callbacks(callbacks);

    let mut remote = repository.find_remote("origin")?;
    remote.push(&[branch_ref], Some(&mut options))?;

    Ok(())
}

pub async fn github_create_pull_request(
    branch_name: &str,
    title: &str,
    body: &str,
) -> Result<(), Box<dyn Error>> {
    let octocrab = octocrab::instance();

    info!("Creating pull request for branch {branch_name}.");

    octocrab
        .pulls(GITHUB_OWNER, GITHUB_REPO)
        .create(title, branch_name, "main")
        .body(body)
        .draft(true)
        .send()
        .await?;

    Ok(())
}
