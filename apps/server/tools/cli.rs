use std::env;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::Result;

const INTERFACE_FILES: &[&str] = &["create", "delete", "get", "update"];
const USECASE_FILES: &[&str] = &["create", "delete", "get", "update"];
const DOMAIN_FILES: &[&str] = &["entity", "errors", "repository"];
const INFRA_FILES: &[&str] =
    &["controllers", "errors", "models", "repository", "routes"];

const DTOS_FILES: &[&str] = &["body", "response", "validators"];

async fn write_if_not_exists(path: &Path, content: &str) -> Result<()> {
    if !path.exists() {
        fs::write(path, content).await?;
    }

    Ok(())
}

async fn create_empty_files(dir: &Path, names: &[&str]) -> Result<()> {
    for name in names {
        fs::write(dir.join(format!("{name}.rs")), "").await?;
    }

    Ok(())
}

async fn create_dir_and_files(dir: &Path, files: &[&str]) -> Result<()> {
    fs::create_dir_all(dir).await?;
    create_empty_files(dir, files).await
}

async fn write_mod_rs(dir: &Path, content: &str) -> Result<()> {
    write_if_not_exists(&dir.join("mod.rs"), content).await
}

async fn append_feature_to_root_mod(feature_name: &str) -> Result<()> {
    let mod_file = Path::new("src/features/mod.rs");
    let new_line = format!("pub mod {};\n", feature_name);

    if mod_file.exists() {
        let contents = fs::read_to_string(mod_file).await?;
        if contents.contains(&new_line)
            || contents.contains(&format!("pub mod {}", feature_name))
        {
            return Ok(());
        }
        let mut updated = contents;
        updated.push_str(&new_line);
        fs::write(mod_file, updated).await?;
    } else {
        fs::write(mod_file, new_line).await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();
    let feature_name = args
        .get(1)
        .expect("Debes proporcionar el nombre de la feature");

    let base_path = PathBuf::from("src/features").join(feature_name);

    let application_path = base_path.join("application");
    let domain_path = base_path.join("domain");
    let infrastructure_path = base_path.join("infrastructure");
    let interfaces_path = application_path.join("interfaces");
    let usecases_path = application_path.join("usecases");
    let dtos_path = infrastructure_path.join("dtos");

    fs::create_dir_all(&base_path).await?;
    create_dir_and_files(&interfaces_path, INTERFACE_FILES).await?;
    create_dir_and_files(&usecases_path, USECASE_FILES).await?;
    create_dir_and_files(&domain_path, DOMAIN_FILES).await?;
    create_dir_and_files(&infrastructure_path, INFRA_FILES).await?;
    create_dir_and_files(&dtos_path, DTOS_FILES).await?;

    write_mod_rs(&base_path, MOD_BASE).await?;
    write_mod_rs(&application_path, MOD_APPLICATION).await?;
    write_mod_rs(&domain_path, MOD_DOMAIN).await?;
    write_mod_rs(&infrastructure_path, MOD_INFRASTRUCTURE).await?;

    append_feature_to_root_mod(feature_name).await?;

    println!("Feature `{}` creada exitosamente", feature_name);
    Ok(())
}

const MOD_BASE: &str = r#"pub mod application;
pub mod domain;
pub mod infrastructure;
"#;

const MOD_APPLICATION: &str = r#"pub mod interfaces {
    mod create;
    mod delete;
    mod get;
    mod update;

    pub use create::*;
    pub use delete::*;
    pub use get::*;
    pub use update::*;
}

pub mod usecases {
    mod create;
    mod delete;
    mod get;
    mod update;

    pub use create::*;
    pub use delete::*;
    pub use get::*;
    pub use update::*;
}
"#;

const MOD_DOMAIN: &str = r#"mod entity;
mod errors;
mod repository;

pub use entity::*;
pub use errors::*;
pub use repository::*;
"#;

const MOD_INFRASTRUCTURE: &str = r#"mod controllers;
mod errors;
mod models;
mod repository;
mod routes;

mod dtos {
    mod body;
    mod response;
    mod validators;

    pub use body::*;
    pub use response::*;
}

pub use repository::*;
"#;
