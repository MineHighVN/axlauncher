// Copyright 2026 MineHighVN, AXLauncher contributors
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use iced::widget::{Space, button, column, container, row, scrollable, text, text_input};
use iced::{Alignment, Border, Color, Element, Length, Theme};

use crate::module::account::entity::{Account, AccountType};
use crate::module::account::repository::AccountRepository;

#[derive(Debug, Clone)]
pub enum Message {
    BackToHome,
    DeleteAccount(Account),
    SelectAccount(Account),

    TypeSelected(AccountType),
    OfflineInputChanged(String),
    ConfirmAdd,
    CancelAdd,
}

pub struct AccountsScreen {
    repo: Arc<AccountRepository>,

    accounts: Vec<Account>,
    active_account: Option<Account>,

    add_mode: Option<AccountType>,
    offline_input: String,
}

impl AccountsScreen {
    pub fn new(repo: Arc<AccountRepository>) -> Self {
        let mut screen = Self {
            repo,
            accounts: Vec::new(),
            active_account: None,
            add_mode: None,
            offline_input: String::new(),
        };

        screen.reload();
        screen
    }

    // Sync from repository to local cache
    fn reload(&mut self) {
        self.accounts = self.repo.get_all();
        self.active_account = self.repo.get_active();
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::BackToHome => {}

            Message::SelectAccount(acc) => {
                self.repo.set_active(Some(acc));
                self.reload();
            }

            Message::DeleteAccount(acc) => {
                self.repo.remove(&acc);
                self.reload();
            }

            Message::TypeSelected(t) => {
                self.add_mode = Some(t);
                self.offline_input.clear();
            }

            Message::OfflineInputChanged(s) => {
                self.offline_input = s;
            }

            Message::CancelAdd => {
                self.add_mode = None;
                self.offline_input.clear();
            }

            Message::ConfirmAdd => {
                if let Some(mode) = self.add_mode {
                    let name = match mode {
                        AccountType::Offline => self.offline_input.clone(),
                        _ => format!("{:?} User", mode), // Simulate other user action
                    };

                    if !name.trim().is_empty() {
                        let new_acc = Account {
                            username: name,
                            account_type: mode,
                        };

                        // Save to repository
                        self.repo.add(new_acc.clone());
                        self.repo.set_active(Some(new_acc));

                        // Reset UI & Reload
                        self.add_mode = None;
                        self.offline_input.clear();
                        self.reload();
                    }
                }
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        // Header
        let title = row![
            button("← Back")
                .on_press(Message::BackToHome)
                .style(button::text),
            text("Manage Accounts").size(24),
        ]
        .spacing(20)
        .align_y(Alignment::Center);

        let content = if let Some(mode) = self.add_mode {
            self.view_add_form(mode)
        } else {
            self.view_list()
        };

        container(column![title, content].spacing(20))
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn view_list(&self) -> Element<'_, Message> {
        let list_content = column(self.accounts.iter().map(|acc| {
            let is_active = self.active_account.as_ref() == Some(acc);

            let status_widget: Element<Message> = if is_active {
                text("Active")
                    .size(14)
                    .style(|_| text::Style {
                        color: Some(iced::Color::from_rgb(0.0, 0.8, 0.0)),
                    })
                    .into()
            } else {
                button("Select")
                    .on_press(Message::SelectAccount(acc.clone()))
                    .padding([5, 10])
                    .style(button::secondary)
                    .into()
            };

            container(
                row![
                    // Text avatar
                    container(
                        text(acc.username.chars().next().unwrap_or('?').to_string()).size(20)
                    )
                    .style(|t: &Theme| container::Style {
                        background: Some(t.extended_palette().primary.base.color.into()),
                        border: Border {
                            radius: 20.0.into(),
                            ..Default::default()
                        },
                        text_color: Some(Color::WHITE),
                        ..Default::default()
                    })
                    .width(40)
                    .height(40)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center),
                    // Info
                    column![
                        text(&acc.username).size(16),
                        text(format!("{:?}", acc.account_type))
                            .size(12)
                            .style(text::secondary),
                    ]
                    .width(Length::Fill),
                    status_widget,
                    button("Remove")
                        .on_press(Message::DeleteAccount(acc.clone()))
                        .padding([5, 10])
                        .style(button::danger)
                ]
                .spacing(15)
                .align_y(Alignment::Center),
            )
            .padding(10)
            .style(move |theme: &Theme| {
                let palette = theme.extended_palette();
                container::Style {
                    background: Some(palette.background.weak.color.into()),
                    border: Border {
                        radius: 8.0.into(),
                        width: 1.0,
                        color: if is_active {
                            palette.primary.base.color
                        } else {
                            palette.background.strong.color
                        },
                    },
                    ..Default::default()
                }
            })
            .into()
        }))
        .spacing(10);

        // Add new account here
        column![
            scrollable(list_content).height(Length::Fill),
            text("Add new account:").size(14).style(text::secondary),
            row![
                button("Microsoft")
                    .on_press(Message::TypeSelected(AccountType::Microsoft))
                    .width(Length::Fill)
                    .style(button::primary),
                button("Offline")
                    .on_press(Message::TypeSelected(AccountType::Offline))
                    .width(Length::Fill)
                    .style(button::secondary),
            ]
            .spacing(10)
        ]
        .spacing(10)
        .into()
    }

    fn view_add_form(&self, mode: AccountType) -> Element<'_, Message> {
        match mode {
            AccountType::Offline => column![
                text("Add Offline Account").size(18),
                text_input("Enter username", &self.offline_input)
                    .on_input(Message::OfflineInputChanged)
                    .on_submit(Message::ConfirmAdd)
                    .padding(10),
                row![
                    button("Cancel")
                        .on_press(Message::CancelAdd)
                        .style(button::text),
                    Space::new().width(Length::Fill),
                    button("Add")
                        .on_press(Message::ConfirmAdd)
                        .style(button::primary),
                ]
                .spacing(20)
            ]
            .max_width(400)
            .spacing(20)
            .into(),

            _ => column![
                text(format!("Login with {:?}", mode)).size(18),
                text("Browser login simulation...").style(text::secondary),
                Space::new().height(20),
                button("Simulate Login Success").on_press(Message::ConfirmAdd),
                button("Cancel")
                    .on_press(Message::CancelAdd)
                    .style(button::text),
            ]
            .spacing(20)
            .into(),
        }
    }
}
