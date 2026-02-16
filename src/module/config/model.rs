// Copyright 2026 MineHighVN, AXLauncher contributors
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub allocated_ram: u32,
    pub java_path: String,
    pub language: String,
    pub theme: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            allocated_ram: 4096,
            java_path: String::from("/usr/bin/java"),
            language: String::from("English"),
            theme: String::from("TokyoNight"),
        }
    }
}
