// Copyright 2026 MineHighVN, AXLauncher contributors
// SPDX-License-Identifier: Apache-2.0

use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::module::launcher::model::Library;

pub struct LauncherRepository {}

impl LauncherRepository {
    /// Returns Java download URL based on OS and Architecture (x86_64, arm64,...)
    // TODO: Implement for other operating system later
    pub fn get_java_download_url() -> &'static str {
        match (std::env::consts::OS, std::env::consts::ARCH) {
            ("macos", "aarch64") => {
                "https://github.com/adoptium/temurin21-binaries/releases/download/jdk-21.0.2%2B13/OpenJDK21U-jdk_aarch64_mac_hotspot_21.0.2_13.tar.gz"
            }
            ("macos", "x86_64") => {
                "https://github.com/adoptium/temurin21-binaries/releases/download/jdk-21.0.2%2B13/OpenJDK21U-jdk_x64_mac_hotspot_21.0.2_13.tar.gz"
            }
            ("windows", _) => {
                "https://github.com/adoptium/temurin21-binaries/releases/download/jdk-21.0.2%2B13/OpenJDK21U-jdk_x64_windows_hotspot_21.0.2_13.zip"
            }
            ("linux", "x86_64") => {
                "https://github.com/adoptium/temurin21-binaries/releases/download/jdk-21.0.2%2B13/OpenJDK21U-jdk_x64_linux_hotspot_21.0.2_13.tar.gz"
            }
            _ => panic!("Unsupported operating system"),
        }
    }

    /// Install file from url and write into a directory
    /// TODO: Refactor to utils later
    pub async fn download_file(url: &str, path: &PathBuf) -> Result<(), String> {
        let response = reqwest::get(url).await.map_err(|e| e.to_string())?;
        let bytes = response.bytes().await.map_err(|e| e.to_string())?;
        fs::write(path, bytes).map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Check library rules to decide if download is needed
    /// Based on current OS (macos, windows, linux)
    /// Return false if OS is not allowed by rules, otherwise true
    pub fn should_download_lib(lib: &Library) -> bool {
        if let Some(rules) = &lib.rules {
            for rule in rules {
                if let Some(os) = &rule.os {
                    #[cfg(target_os = "macos")]
                    if os.name != "osx" && rule.action == "allow" {
                        return false;
                    }

                    #[cfg(target_os = "windows")]
                    if os.name != "windows" && rule.action == "allow" {
                        return false;
                    }

                    #[cfg(target_os = "linux")]
                    if os.name != "linux" && rule.action == "allow" {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// Download and extract java
    pub async fn download_and_extract_java(dest_dir: &PathBuf) -> Result<PathBuf, String> {
        fs::create_dir_all(&dest_dir.join("java_runtime")).map_err(|e| e.to_string())?;

        let url = Self::get_java_download_url();
        let tar_gz_path = dest_dir.join("../java_temp.tar.gz");

        Self::download_file(url, &tar_gz_path).await?;

        // Extract to java_runtime directory
        let tar_gz = fs::File::open(&tar_gz_path).map_err(|e| e.to_string())?;
        let tar = flate2::read::GzDecoder::new(tar_gz);
        let mut archive = tar::Archive::new(tar);

        archive.unpack(dest_dir).map_err(|e| e.to_string())?;

        // Delete temp file
        let _ = fs::remove_file(tar_gz_path);

        // Find executable file
        // TODO: Create for other operating system later
        let java_exec = dest_dir.join("jdk-21.0.2+13/Contents/Home/bin/java");

        // Grant file permissions for Linux, MacOS (Unix Like)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            let mut perms = fs::metadata(&java_exec)
                .map_err(|e| e.to_string())?
                .permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&java_exec, perms).map_err(|e| e.to_string())?;
        }

        Ok(java_exec)
    }

    /// Find path to installed java runtime (binary - executable)
    /// Look in {current_dir}/java_runtime
    /// Return path to java executable
    pub fn find_java(current_dir: &Path) -> PathBuf {
        // Find path to java_runtime
        let java_runtime_dir = current_dir.join("java_runtime");

        // Find java in directory java_runtime
        let java_exec = java_runtime_dir.join("jdk-21.0.2+13/Contents/Home/bin/java");

        java_exec
    }
}
