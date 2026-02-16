// Copyright 2026 MineHighVN, AXLauncher contributors
// SPDX-License-Identifier: Apache-2.0

use std::sync::{LazyLock, RwLock};

use iced::{Color, Theme};

use crate::state::State;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct ThemePalette {
    pub bg_main: Color,
    pub bg_card: Color,
    pub primary: Color,
    pub secondary: Color,
    pub text_main: Color,
    pub text_subtle: Color,
    pub accent: Color,
    pub danger: Color,
    pub success: Color,
    pub warning: Color,
}

impl ThemePalette {
    pub fn tokyo_night() -> Self {
        Self {
            bg_main: Color::from_rgb8(26, 27, 38),
            bg_card: Color::from_rgb8(36, 40, 59),
            primary: Color::from_rgb8(122, 162, 247),
            secondary: Color::from_rgb8(187, 154, 247),
            text_main: Color::from_rgb8(169, 177, 214),
            text_subtle: Color::from_rgb8(86, 95, 137),
            accent: Color::from_rgb8(255, 158, 100),
            danger: Color::from_rgb8(247, 118, 142),
            success: Color::from_rgb8(158, 206, 106),
            warning: Color::from_rgb8(224, 175, 104),
        }
    }

    pub fn tokyo_night_light() -> Self {
        Self {
            bg_main: Color::from_rgb8(213, 214, 219),
            bg_card: Color::from_rgb8(203, 204, 210),
            primary: Color::from_rgb8(52, 90, 188),
            secondary: Color::from_rgb8(136, 80, 191),
            text_main: Color::from_rgb8(55, 59, 77),
            text_subtle: Color::from_rgb8(101, 107, 133),
            accent: Color::from_rgb8(143, 78, 40),
            danger: Color::from_rgb8(247, 118, 142),
            success: Color::from_rgb8(158, 206, 106),
            warning: Color::from_rgb8(224, 175, 104),
        }
    }
}

pub fn theme(_state: &State) -> Theme {
    _state.theme.clone()
}

pub static CURRENT_PALETTE: LazyLock<RwLock<ThemePalette>> =
    LazyLock::new(|| RwLock::new(ThemePalette::tokyo_night()));
