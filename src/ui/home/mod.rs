// Copyright 2026 MineHighVN, AXLauncher contributors
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use iced::widget::{Space, column, container, pick_list, row, text};
use iced::{Alignment, Font, Length, Task};

use crate::common::app_ui::AppUI;
use crate::module::account::repository::AccountRepository;
use crate::module::launcher::service::{LaunchArgs, LauncherService};
use crate::module::mojang::entity::MinecraftVersion;
use crate::module::mojang::repository::MojangRepository;

#[derive(Debug, Clone)]
pub enum Message {
    PlayPressed,
    LaunchFinished,
    VersionSelected(MinecraftVersion),
    VersionsLoaded(Result<Vec<MinecraftVersion>, String>),
}

pub struct HomeScreen {
    pub versions: Vec<MinecraftVersion>,
    pub selected_version: Option<MinecraftVersion>,
    pub error: Option<String>,
    pub account_repo: Arc<AccountRepository>,
}

impl HomeScreen {
    pub fn new(
        mojang_repo: Arc<MojangRepository>,
        account_repo: Arc<AccountRepository>,
    ) -> (Self, Task<Message>) {
        let repo_clone = mojang_repo.clone();
        (
            Self {
                versions: Vec::new(),
                selected_version: None,
                error: None,
                account_repo,
            },
            Task::perform(
                async move { repo_clone.get_all_versions().await },
                Message::VersionsLoaded,
            ),
        )
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::VersionsLoaded(Ok(v)) => {
                self.versions = v;
                self.error = None;
            }
            Message::VersionsLoaded(Err(e)) => {
                self.error = Some(e);
            }
            Message::VersionSelected(v) => {
                self.selected_version = Some(v);
            }
            Message::PlayPressed => {
                if let Some(v) = &self.selected_version {
                    let Some(active_user) = self.account_repo.get_active() else {
                        // TODO: Show user a warning

                        println!("No active account");
                        return Task::none();
                    };

                    println!(
                        "Playing version {} with user {}",
                        v.id, active_user.username
                    );

                    if let Some(version) = &self.selected_version {
                        return Task::perform(
                            LauncherService::launch(
                                LaunchArgs {
                                    username: active_user.username.clone(),
                                    ..Default::default()
                                },
                                version.clone(),
                            ),
                            |_| Message::LaunchFinished,
                        );
                    }
                }
            }
            Message::LaunchFinished => {
                println!("Game launched successfully!");
            }
        }

        Task::none()
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        let main_msg = if let Some(err) = &self.error {
            format!("Error: {}", err)
        } else {
            "Welcome to the Minecraft Launcher!".to_string()
        };

        let content = column![
            row![column![
                text("Welcome back!")
                    .font(Font {
                        weight: iced::font::Weight::Bold,
                        ..Default::default()
                    })
                    .size(24),
                text("Ready to start your adventure?").size(16),
                Space::new().width(Length::Fill),
            ],],
            container(text(main_msg))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill),
            row![
                pick_list(
                    self.versions.as_slice(),
                    self.selected_version.as_ref(),
                    Message::VersionSelected
                )
                .placeholder("Minecraft Version")
                .width(Length::Fill),
                AppUI::button("Play")
                    .on_press(Message::PlayPressed)
                    .padding([10, 20])
                    .build(),
            ]
            .spacing(10)
            .align_y(Alignment::Center)
        ];

        content.into()
    }
}
