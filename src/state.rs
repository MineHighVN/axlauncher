// Copyright 2026 MineHighVN, AXLauncher contributors
// SPDX-License-Identifier: Apache-2.0

use iced::Theme;

use crate::{
    common::sidebar::{self},
    ui,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Home,
    Instances,
    Settings,
    Accounts,
}

pub struct SavedScreen {
    pub home: ui::home::HomeScreen,
    pub settings: ui::settings::SettingsScreen,
    pub accounts: ui::accounts::AccountsScreen,
}

impl SavedScreen {
    pub fn new(
        home: ui::home::HomeScreen,
        settings: ui::settings::SettingsScreen,
        accounts: ui::accounts::AccountsScreen,
    ) -> Self {
        Self {
            home,
            settings,
            accounts,
        }
    }
}

pub struct State {
    pub current_page: Page,
    pub theme: Theme,
    pub saved_screen: SavedScreen,
}

impl State {
    pub fn new(theme: Theme, saved_screen: SavedScreen) -> Self {
        Self {
            current_page: Page::Home,
            theme,
            saved_screen,
        }
    }
}

pub enum Message {
    Home(ui::home::Message),
    Settings(ui::settings::Message),
    Accounts(ui::accounts::Message),
    Sidebar(sidebar::Message),
}
