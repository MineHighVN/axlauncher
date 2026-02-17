// Copyright 2026 MineHighVN, AXLauncher contributors
// SPDX-License-Identifier: Apache-2.0

use crate::module::mojang::{entity::MinecraftVersion, model::VersionManifest};

pub struct MojangRepository;

impl MojangRepository {
    pub fn new() -> Self {
        return Self {};
    }

    pub async fn get_all_versions(&self) -> Result<Vec<MinecraftVersion>, String> {
        let url = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

        let response = reqwest::get(url)
            .await
            .map_err(|e| e.to_string())?
            .json::<VersionManifest>()
            .await
            .map_err(|e| e.to_string())?;

        Ok(response
            .versions
            .into_iter()
            .map(|v| MinecraftVersion {
                id: v.id,
                version_type: v.r#type,
                url: Some(v.url),
            })
            .collect())
    }
}
