// Copyright 2026 MineHighVN, AXLauncher contributors
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct VersionDetail {
    pub mainClass: String,
    pub downloads: Downloads,
    pub assetIndex: AssetIndex,
    pub libraries: Vec<Library>,
}

#[derive(Deserialize, Debug)]
pub struct Downloads {
    pub client: DownloadInfo,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct DownloadInfo {
    pub url: String,
    pub size: u64,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct AssetIndex {
    pub id: String,
    pub url: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Library {
    pub downloads: Option<LibDownloads>,
    pub name: String,
    pub rules: Option<Vec<Rule>>,
}

#[derive(Deserialize, Debug)]
pub struct LibDownloads {
    pub artifact: Option<LibArtifact>,
}

#[derive(Deserialize, Debug)]
pub struct LibArtifact {
    pub path: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Rule {
    pub action: String,
    pub os: Option<OsRule>,
}

#[derive(Deserialize, Debug)]
pub struct OsRule {
    pub name: String,
}
