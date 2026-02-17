// Copyright 2026 MineHighVN, AXLauncher contributors
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;
use std::sync::Arc;

use iced::widget::{Space, button, column, container, row, scrollable, text};
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
    LocalVersionsLoaded(Vec<MinecraftVersion>),
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
            Task::batch([
                Task::perform(
                    async move { repo_clone.get_all_versions().await },
                    Message::VersionsLoaded,
                ),
                Task::done(Message::LocalVersionsLoaded(
                    // FIXME: HANDLE ERROR
                    LauncherService::get_local_minecraft_versions().unwrap(),
                )),
            ]),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::VersionsLoaded(Ok(remote_versions)) => {
                // Use HashSet to make the merging process faster
                let mut version_set: HashSet<String> =
                    self.versions.iter().map(|v| v.id.clone()).collect();

                for v in remote_versions {
                    if !version_set.contains(&v.id) {
                        version_set.insert(v.id.clone());
                        self.versions.push(v);
                    }
                }
                self.error = None;
            }
            Message::VersionsLoaded(Err(e)) => {
                self.error = Some(e);
            }
            Message::VersionSelected(v) => {
                self.selected_version = Some(v);
            }
            Message::LocalVersionsLoaded(local_versions) => {
                // Use HashSet to make the merging process faster
                let existing_ids: std::collections::HashSet<String> =
                    self.versions.iter().map(|v| v.id.clone()).collect();

                let mut to_add: Vec<MinecraftVersion> = local_versions
                    .into_iter()
                    .filter(|v| !existing_ids.contains(&v.id))
                    .collect();

                to_add.append(&mut self.versions);
                self.versions = to_add;
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
                            |e| {
                                if e.is_err() {
                                    println!("Error: {:?}", e);
                                    Message::LaunchFinished
                                } else {
                                    Message::LaunchFinished
                                }
                            },
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
        let header = column![
            text("Welcome back!").size(28).font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            text("Ready to start your adventure?").size(14)
        ]
        .spacing(5);

        let version_list = scrollable(
            column(
                self.versions
                    .iter()
                    .map(|v| {
                        let is_selected = self
                            .selected_version
                            .as_ref()
                            .map(|sv| &sv.id == &v.id)
                            .unwrap_or(false);

                        let content = row![
                            column![
                                text(&v.id).size(16).font(Font {
                                    weight: iced::font::Weight::Semibold,
                                    ..Default::default()
                                }),
                                text(&v.version_type).size(12),
                            ]
                            .spacing(2),
                            Space::new().width(Length::Fill),
                            if v.available {
                                Some(
                                    container(text("READY"))
                                        .padding([4, 8])
                                        .style(container::rounded_box),
                                )
                            } else {
                                None
                            }
                        ]
                        .align_y(Alignment::Center)
                        .padding(12);

                        button(content)
                            .width(Length::Fill)
                            .on_press(Message::VersionSelected(v.clone()))
                            .style(if is_selected {
                                button::primary
                            } else {
                                button::subtle
                            })
                            .into()
                    })
                    .collect::<Vec<_>>(),
            )
            .spacing(10),
        )
        .width(Length::Fill)
        .height(Length::Fill);

        let footer = container(
            row![
                column![
                    text("Selected Version").size(12),
                    text(
                        self.selected_version
                            .as_ref()
                            .map(|v| v.id.as_str())
                            .unwrap_or("None")
                    )
                    .size(16)
                    .font(Font {
                        weight: iced::font::Weight::Bold,
                        ..Default::default()
                    }),
                ]
                .spacing(2),
                Space::new().width(Length::Fill),
                AppUI::button("LAUNCH GAME")
                    .on_press(Message::PlayPressed)
                    .padding([12, 40])
                    .build(),
            ]
            .align_y(Alignment::Center),
        )
        .padding(20)
        .width(Length::Fill)
        .style(container::rounded_box);

        column![
            header,
            Space::new().height(20),
            text("VERSION SELECTION").size(12).font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            container(version_list)
                .height(Length::FillPortion(7))
                .width(Length::Fill),
            if let Some(err) = &self.error {
                container(text(err)).padding(10).width(Length::Fill)
            } else {
                container(Space::new().height(Length::Shrink))
            },
            Space::new().height(Length::FillPortion(1)),
            footer
        ]
        .padding(30)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
