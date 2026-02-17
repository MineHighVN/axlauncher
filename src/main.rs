// Copyright 2026 MineHighVN, AXLauncher contributors
// SPDX-License-Identifier: Apache-2.0

mod common;
mod module;
mod state;
mod theme;
mod ui;

use std::sync::Arc;

use iced::{
    Element, Task, Theme,
    widget::{container, row, text},
};

use crate::{
    common::sidebar::{self, app_sidebar},
    module::{
        account::repository::AccountRepository, config::repository::ConfigRepository,
        mojang::repository::MojangRepository,
    },
    state::*,
};

fn update(state: &mut State, message: Message) -> Task<Message> {
    #[allow(unreachable_patterns)]
    match message {
        Message::Home(message) => state.saved_screen.home.update(message).map(Message::Home),
        Message::Settings(message) => {
            match &message {
                ui::settings::Message::ThemeChanged(new_theme) => {
                    // Update state for iced
                    state.theme = new_theme.clone();

                    // Update Global Pallete for AppUI
                    {
                        let mut palette = crate::theme::CURRENT_PALETTE.write().unwrap();
                        *palette = match new_theme {
                            iced::Theme::TokyoNightLight => {
                                crate::theme::ThemePalette::tokyo_night_light()
                            }
                            _ => crate::theme::ThemePalette::tokyo_night(),
                        };
                    }
                }
                ui::settings::Message::MinecraftRootDirChanged(dir) => {
                    println!("directory: {}", dir);
                }
                _ => {}
            };

            state
                .saved_screen
                .settings
                .update(message)
                .map(Message::Settings)
        }
        Message::Accounts(message) => {
            state.saved_screen.accounts.update(message);
            Task::none()
        }
        Message::Sidebar(sidebar::Message::PageSelected(page)) => {
            state.current_page = page;

            Task::none()
        }
        _ => Task::none(),
    }
}

fn view(state: &State) -> Element<'_, Message> {
    let sidebar = app_sidebar(state.current_page).map(Message::Sidebar);

    let content: Element<'_, Message> = match &state.current_page {
        Page::Home => state.saved_screen.home.view().map(Message::Home),
        Page::Settings => state.saved_screen.settings.view().map(Message::Settings),
        Page::Accounts => state.saved_screen.accounts.view().map(Message::Accounts),
        _ => text("Page not found").into(),
    };

    row![
        sidebar,
        container(content)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .padding(20)
    ]
    .into()
}

fn new() -> (State, Task<Message>) {
    let config = ConfigRepository::load();

    // Handle theme
    let theme = match config.theme.as_str() {
        "TokyoNight" => Theme::TokyoNight,
        "TokyoNightLight" => Theme::TokyoNightLight,
        _ => Theme::TokyoNight,
    };

    let mojang_repo = Arc::new(MojangRepository::new());
    let account_repo = Arc::new(AccountRepository::new());

    let (home_screen, home_task) =
        ui::home::HomeScreen::new(mojang_repo.clone(), account_repo.clone());

    let settings_screen = ui::settings::SettingsScreen::new(theme.clone());

    let accounts_screen = ui::accounts::AccountsScreen::new(account_repo.clone());

    let saved_screen = SavedScreen::new(home_screen, settings_screen, accounts_screen);

    let state = State::new(theme, saved_screen);

    (state, home_task.map(|home_msg| Message::Home(home_msg)))
}

fn main() -> iced::Result {
    iced::application(new, update, view)
        .theme(theme::theme)
        .run()
}
