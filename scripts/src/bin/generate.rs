use std::{env, error::Error};

use scripts::{UPSTREAM_GIT_REF, generate::generate};

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    generate(UPSTREAM_GIT_REF, env::current_dir()?.as_path())
}
