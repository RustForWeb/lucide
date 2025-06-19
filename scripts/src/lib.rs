#![feature(exit_status_error)]

mod framework;
mod frameworks;
pub mod generate;
mod metadata;
pub mod repository;

pub const GIT_URL: &str = "https://github.com/RustForWeb/lucide.git";
pub const GITHUB_OWNER: &str = "RustForWeb";
pub const GITHUB_REPO: &str = "lucide";

pub const UPSTREAM_GIT_URL: &str = "https://github.com/lucide-icons/lucide.git";
pub const UPSTREAM_GIT_REF: &str = "0.518.0";
pub const UPSTREAM_GITHUB_URL: &str = "https://github.com/lucide-icons/lucide";
