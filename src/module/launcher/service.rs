// Copyright 2026 MineHighVN, AXLauncher contributors
// SPDX-License-Identifier: Apache-2.0

use std::io::Error;
use std::path::PathBuf;

use std::fs;
use std::path::Path;
use std::process::Command;

use crate::module::config::repository::ConfigRepository;
use crate::module::launcher::model::VersionDetail;
use crate::module::launcher::repository::LauncherRepository;
use crate::module::mojang::entity::MinecraftVersion;

struct LauncherPaths {
    root_dir: PathBuf,
    version_dir: PathBuf,
    libraries_dir: PathBuf,
    assets_dir: PathBuf,
    version_json: PathBuf,
    client_jar: PathBuf,
}

impl LauncherPaths {
    /// Create new instance and setup paths for Minecraft
    /// Based on current_dir, root will be at {current_dir}/../minecraft_data
    /// Initialize paths for version, jar, libraries and assets
    /// TODO: Add path for release (%APPDATA%/.minecraft, ~/.minecraft) later
    /// TODO: Path is editable, not fixed
    fn new(version_id: &str, minecraft_root_dir: PathBuf) -> Result<Self, String> {
        let root_dir = minecraft_root_dir;
        let version_dir = root_dir.join("versions").join(version_id);

        Ok(Self {
            version_json: version_dir.join(format!("{}.json", version_id)),
            client_jar: version_dir.join(format!("{}.jar", version_id)),
            libraries_dir: root_dir.join("libraries"),
            assets_dir: root_dir.join("assets"),
            version_dir,
            root_dir,
        })
    }

    /// Ensure all dependency directories are exists
    /// Missing directories will be created
    fn ensure_directories(&self) -> Result<(), String> {
        fs::create_dir_all(&self.version_dir).map_err(|e| e.to_string())?;
        fs::create_dir_all(&self.libraries_dir).map_err(|e| e.to_string())?;
        fs::create_dir_all(&self.assets_dir).map_err(|e| e.to_string())?;
        Ok(())
    }
}

// This struct defines arguments for launching minecraft
pub struct LaunchArgs {
    pub username: String,
    pub uuid: String,
    pub access_token: String,
}

impl Default for LaunchArgs {
    // TODO: implement uuid and access token
    fn default() -> Self {
        Self {
            username: "".to_owned(),
            uuid: "00000000-0000-0000-0000-000000000000".to_owned(),
            access_token: "0".to_owned(),
        }
    }
}

pub struct LauncherService {}

impl LauncherService {
    fn get_minecraft_root_dir() -> Result<PathBuf, Error> {
        let config = ConfigRepository::load();

        let minecraft_root_dir = config.minecraft_root_dir;

        return Ok(minecraft_root_dir.into());
    }

    /// Lauch minecraft
    pub async fn launch(launch_args: LaunchArgs, version: MinecraftVersion) -> Result<(), String> {
        let minecraft_root_dir = Self::get_minecraft_root_dir().map_err(|e| e.to_string())?;

        // Create path
        let paths = LauncherPaths::new(&version.id, minecraft_root_dir)?;
        paths.ensure_directories()?;

        // Install Metadata
        let detail = Self::prepare_version_metadata(&version, &paths).await?;

        // Install all libraries (eg. jar, libraries)
        Self::prepare_dependencies(&detail, &paths).await?;

        // Build classpath
        let classpath = Self::build_classpath(&detail, &paths);

        // Find java
        let current_dir = std::env::current_dir().map_err(|e| e.to_string())?;
        let java_path = Self::get_java_path(&current_dir.join("..")).await?;

        // Run minecraft
        Self::run_minecraft(java_path, paths, detail, classpath, version.id, launch_args)
    }

    /// Prepare version metadata
    /// Install if not found
    async fn prepare_version_metadata(
        version: &MinecraftVersion,
        paths: &LauncherPaths,
    ) -> Result<VersionDetail, String> {
        if !version.available {
            let Some(version_url) = &version.url else {
                return Err("Unknown error".to_owned());
            };

            if !paths.version_json.exists() {
                LauncherRepository::download_file(&version_url, &paths.version_json).await?;
            }
        }

        let content = fs::read_to_string(&paths.version_json).map_err(|e| e.to_string())?;
        let mut detail: VersionDetail =
            serde_json::from_str(&content).map_err(|e| e.to_string())?;

        // Handle version inherits
        if let Some(parent_id) = &detail.inherits_from {
            let root_dir = paths.root_dir.clone();
            let parent_json_path = root_dir
                .join("versions")
                .join(parent_id)
                .join(format!("{}.json", parent_id));

            if parent_json_path.exists() {
                let parent_content =
                    fs::read_to_string(parent_json_path).map_err(|e| e.to_string())?;
                let parent_detail: VersionDetail =
                    serde_json::from_str(&parent_content).map_err(|e| e.to_string())?;

                // Combind libraries
                let mut all_libraries = detail.libraries;
                all_libraries.extend(parent_detail.libraries);
                detail.libraries = all_libraries;

                if detail.downloads.is_none() {
                    detail.downloads = parent_detail.downloads;
                }

                if detail.assetIndex.is_none() {
                    detail.assetIndex = parent_detail.assetIndex;
                }
            }
        }

        Ok(detail)
    }

    /// Prepare all dependencies
    /// Missing dependencies will be installed automatically
    async fn prepare_dependencies(
        detail: &VersionDetail,
        paths: &LauncherPaths,
    ) -> Result<(), String> {
        // Install client jar
        if paths.client_jar.try_exists().map_err(|e| e.to_string())? == false {
            if let Some(downloads) = &detail.downloads {
                LauncherRepository::download_file(&downloads.client.url, &paths.client_jar).await?;
            }
        }

        // Install libraries
        for lib in &detail.libraries {
            if !LauncherRepository::should_download_lib(lib) {
                continue;
            }

            if let Some(artifact) = lib.downloads.as_ref().and_then(|d| d.artifact.as_ref()) {
                let lib_path = paths.libraries_dir.join(&artifact.path);
                if !lib_path.exists() {
                    if let Some(parent) = lib_path.parent() {
                        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
                    }
                    LauncherRepository::download_file(&artifact.url, &lib_path).await?;
                }
            }
        }
        Ok(())
    }

    // Convert java library name to path
    fn name_to_path(name: &String) -> String {
        let parts: Vec<&str> = name.split(':').collect();
        if parts.len() < 3 {
            return name.clone();
        }

        let group = parts[0].replace('.', "/");
        let artifact = parts[1];
        let version = parts[2];

        format!(
            "{}/{}/{}/{}-{}.jar",
            group, artifact, version, artifact, version
        )
    }

    /// Build classpath
    fn build_classpath(detail: &VersionDetail, paths: &LauncherPaths) -> String {
        let mut entries = vec![paths.client_jar.to_str().unwrap().to_string()];

        for lib in &detail.libraries {
            if !LauncherRepository::should_download_lib(lib) {
                continue;
            }
            let relative_path =
                if let Some(artifact) = lib.downloads.as_ref().and_then(|d| d.artifact.as_ref()) {
                    Some(artifact.path.clone())
                } else {
                    Some(Self::name_to_path(&lib.name))
                };

            if let Some(path) = relative_path {
                let lib_path = paths.libraries_dir.join(path);
                if let Some(p_str) = lib_path.to_str() {
                    entries.push(p_str.to_string());
                }
            }
        }

        #[cfg(target_os = "windows")]
        let sep = ";";
        #[cfg(not(target_os = "windows"))]
        let sep = ":";

        entries.join(sep)
    }

    /// Spawn a new command process to run Minecraft by using Java runtime
    fn run_minecraft(
        java: PathBuf,
        paths: LauncherPaths,
        detail: VersionDetail,
        classpath: String,
        id: String,
        args: LaunchArgs,
    ) -> Result<(), String> {
        let mut cmd = Command::new(java);
        cmd.current_dir(&paths.root_dir);

        #[cfg(target_os = "macos")]
        cmd.arg("-XstartOnFirstThread");

        let asset_index_id = detail
            .assetIndex
            .as_ref()
            .map(|a| a.id.as_str())
            .unwrap_or("legacy");

        cmd.args([
            "-Xmx2G",
            "-cp",
            &classpath,
            &detail.mainClass,
            "--version",
            &id,
            "--gameDir",
            ".",
            "--assetsDir",
            paths.assets_dir.to_str().unwrap(),
            "--assetIndex",
            &asset_index_id,
            "--username",
            &args.username,
            "--uuid",
            &args.uuid,
            "--accessToken",
            &args.access_token,
            "--userType",
            "legacy",
        ]);

        cmd.spawn()
            .map(|_| ())
            .map_err(|e| format!("Spawn error: {}", e))
    }

    /// Returns java runtime path (binary - executable)
    /// Searches for binary at `{find_dir}/java_runtime`
    /// Install if not found
    async fn get_java_path(find_dir: &Path) -> Result<PathBuf, String> {
        let java_path = LauncherRepository::find_java(find_dir);
        if java_path.exists() {
            Ok(java_path)
        } else {
            let download_java_path = LauncherRepository::download_and_extract_java(
                &find_dir.join("java_runtime").into(),
            )
            .await
            .unwrap();
            Ok(download_java_path)
        }
    }

    pub fn get_local_minecraft_versions() -> Result<Vec<MinecraftVersion>, Error> {
        let minecraft_root_dir = Self::get_minecraft_root_dir()?.join("versions");

        let list_dir = fs::read_dir(minecraft_root_dir)?;

        let minecraft_versions: Vec<MinecraftVersion> = list_dir
            .map(|v| MinecraftVersion {
                id: v.unwrap().file_name().into_string().unwrap(),
                version_type: "modded".to_owned(),
                url: None,
                available: true,
            })
            .collect();

        Ok(minecraft_versions)
    }
}
