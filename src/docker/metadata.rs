use std::path::PathBuf;

use log::debug;
use serde::Deserialize;

use crate::{
    docker::{DockerPackage, TargetDir},
    Dependencies, Error, Result,
};

#[derive(Debug, Clone, Deserialize)]
pub struct DockerMetadata {
    pub deps_hash: Option<String>,
    pub base: String,
    pub copy_dest_dir: String,
    pub env: Option<Vec<EnvironmentVariable>>,
    pub run: Option<Vec<String>>,
    pub expose: Option<Vec<i32>>,
    pub workdir: Option<String>,
    pub extra_copies: Option<Vec<CopyCommand>>,
    pub extra_commands: Option<Vec<String>>,
}

impl DockerMetadata {
    pub fn into_dist_target(
        self,
        target_dir: &PathBuf,
        package: &cargo_metadata::Package,
        dependencies: Dependencies,
    ) -> Result<DockerPackage> {
        debug!("Package has a Docker target distribution.");

        let docker_dir = target_dir.join("docker").join(&package.name);

        let binaries: Vec<_> = package
            .targets
            .iter()
            .filter_map(|target| {
                if target.kind.contains(&"bin".to_string()) {
                    Some(target.name.clone())
                } else {
                    None
                }
            })
            .collect();

        if binaries.is_empty() {
            return Err(Error::new("package contain no binaries").with_explanation(format!("Building a Docker image requires at least one binary but the package {} does not contain any.", package.id)));
        }

        debug!(
            "Package contains the following binaries: {}",
            binaries.join(", ")
        );

        Ok(DockerPackage {
            name: package.name.clone(),
            version: package.version.to_string(),
            toml_path: package.manifest_path.to_string(),
            binaries,
            metadata: self,
            dependencies,
            target_dir: TargetDir {
                binary_dir: target_dir.clone(),
                docker_dir,
            },
            package: package.clone(),
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvironmentVariable {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CopyCommand {
    pub source: String,
    pub destination: String,
}
