use std::{env, error::Error};

use log::info;
use scripts::{
    GIT_URL, UPSTREAM_GIT_REF, UPSTREAM_GIT_URL, UPSTREAM_GITHUB_URL,
    generate::generate,
    repository::{
        git_checkout, git_commit, git_create_branch, git_has_remote_branch, git_push,
        git_version_tags, github_create_pull_request,
    },
};
use semver::Version;
use tempfile::tempdir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    octocrab::initialise(
        octocrab::OctocrabBuilder::new()
            .personal_token(env::var("GITHUB_TOKEN")?)
            .build()?,
    );

    let upstream_repository_path = tempdir()?;
    let upstream_repository = git_checkout(
        &upstream_repository_path,
        UPSTREAM_GIT_URL,
        UPSTREAM_GIT_REF,
    )?;

    let current_version = Version::parse(UPSTREAM_GIT_REF.trim_start_matches("v"))?;
    let versions = git_version_tags(&upstream_repository)?;

    let index = versions
        .iter()
        .position(|version| *version == current_version);

    let new_versions = match index {
        Some(index) => &versions[(index + 1)..],
        None => &versions,
    };

    info!("Current version: {current_version}");

    if let Some(new_version) = new_versions.iter().next() {
        info!("New version: {new_version}");

        let next_version = new_version.to_string();
        let branch_name = format!("upstream/v{next_version}");

        let repository_temp_dir = tempdir()?;
        let repository_path = repository_temp_dir.path();

        let repository = git_checkout(repository_path, GIT_URL, "main")?;

        if git_has_remote_branch(&repository, &branch_name) {
            info!("Branch {branch_name} already exists.");
            return Ok(());
        }

        git_create_branch(&repository, &branch_name)?;

        generate(&next_version, repository_path)?;

        let message = format!("feat: update to upstream v{new_version}");
        git_commit(&repository, &message)?;

        git_push(&repository, &branch_name)?;

        let body = format!(
            "Update to upstream [Lucide v{new_version}]({UPSTREAM_GITHUB_URL}/releases/tag/{new_version})."
        );

        github_create_pull_request(&branch_name, &message, &body).await?;
    } else {
        info!("No new version.");
    }

    Ok(())
}
