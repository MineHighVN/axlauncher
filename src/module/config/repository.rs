// Copyright 2026 MineHighVN, AXLauncher contributors
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;

use crate::module::config::model::AppConfig;

pub struct ConfigRepository {}

impl ConfigRepository {
    pub fn load() -> AppConfig {
        #[cfg(debug_assertions)]
        {
            let path = PathBuf::from("../axlauncher.conf");
            confy::load_path(&path).unwrap_or_default()
        }

        #[cfg(not(debug_assertions))]
        {
            confy::load("axlauncher", None).unwrap_or_default()
        }
    }

    pub fn save(config: AppConfig) {
        #[cfg(debug_assertions)]
        {
            let path = PathBuf::from("../axlauncher.conf");
            let _ = confy::store_path(&path, config);
        }

        #[cfg(not(debug_assertions))]
        {
            let _ = confy::store("axlauncher", None, config);
        }
    }
}
