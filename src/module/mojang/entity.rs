// Copyright 2026 MineHighVN, AXLauncher contributors
// SPDX-License-Identifier: Apache-2.0

use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MinecraftVersion {
    pub id: String,
    pub version_type: String,
    pub url: String,
}

impl fmt::Display for MinecraftVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}
