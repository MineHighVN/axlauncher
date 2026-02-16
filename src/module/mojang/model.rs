// Copyright 2026 MineHighVN, AXLauncher contributors
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct VersionManifest {
    pub versions: Vec<ApiVersion>,
}

#[derive(Deserialize, Debug)]
pub struct ApiVersion {
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub url: String,
}
