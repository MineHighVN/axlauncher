// Copyright 2026 MineHighVN, AXLauncher contributors
// SPDX-License-Identifier: Apache-2.0

use iced::widget::{
    Space, button, column, container, pick_list, row, scrollable, slider, text, text_input,
};
use iced::{
    Alignment, Background, Border, Color, Element, Font, Length, Shadow, Task, Theme, Vector,
};

use crate::module::config::model::AppConfig;
use crate::module::config::repository::ConfigRepository;

#[derive(Debug, Clone)]
pub enum Message {
    RamChanged(u32),
    JavaPathChanged(String),
    LanguageChanged(String),
    ThemeChanged(Theme),
    CheckForUpdates,
    OpenGithub,
}

pub struct SettingsScreen {
    pub allocated_ram: u32,
    pub java_path: String,
    pub selected_language: String,
    pub current_theme: Theme,
}

impl SettingsScreen {
    pub fn new(theme: Theme) -> Self {
        Self {
            allocated_ram: 4096,
            java_path: String::from("/usr/bin/java"),
            selected_language: String::from("English"),
            current_theme: theme,
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::RamChanged(ram) => self.allocated_ram = ram,
            Message::JavaPathChanged(path) => self.java_path = path,
            Message::LanguageChanged(lang) => self.selected_language = lang,
            Message::ThemeChanged(theme) => self.current_theme = theme,
            _ => {}
        }

        let config = AppConfig {
            allocated_ram: self.allocated_ram,
            java_path: self.java_path.clone(),
            language: self.selected_language.clone(),
            theme: format!("{:?}", self.current_theme),
        };

        let _ = ConfigRepository::save(config);

        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let header = column![
            text("Settings").size(36).font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            text("Customize your launcher experience")
                .size(14)
                .style(|theme: &Theme| text::Style {
                    color: Some(theme.extended_palette().secondary.base.text)
                }),
        ]
        .spacing(5);

        let content = column![
            header,
            Space::new().height(30), // Spacer
            self.view_section_title("GAMEPLAY"),
            self.view_settings_card(self.view_game_settings_content()),
            Space::new().height(20), // Spacer
            self.view_section_title("APPEARANCE & SYSTEM"),
            self.view_settings_card(self.view_launcher_settings_content()),
            Space::new().height(Length::Fill),
            self.view_footer_section(),
        ]
        .spacing(10)
        .max_width(800);

        container(scrollable(content))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(40)
            .center_x(Length::Fill)
            .style(|theme: &Theme| {
                let palette = theme.extended_palette();
                container::Style {
                    background: Some(Background::Color(palette.background.base.color)),
                    ..Default::default()
                }
            })
            .into()
    }

    fn view_settings_card<'a>(&self, content: Element<'a, Message>) -> Element<'a, Message> {
        container(content)
            .padding(24)
            .width(Length::Fill)
            .style(|theme: &Theme| {
                let palette = theme.extended_palette();
                container::Style {
                    background: Some(Background::Color(palette.background.weak.color)),
                    border: Border {
                        width: 1.0,
                        color: palette.background.strong.color,
                        radius: 16.0.into(),
                    },
                    shadow: Shadow {
                        color: Color::BLACK.scale_alpha(0.05),
                        offset: Vector::new(0.0, 4.0),
                        blur_radius: 12.0,
                    },
                    ..Default::default()
                }
            })
            .into()
    }

    // Subtitle for each section
    fn view_section_title<'a>(&self, label: &'a str) -> Element<'a, Message> {
        text(label)
            .size(12)
            .font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            })
            .style(|theme: &Theme| text::Style {
                color: Some(theme.extended_palette().secondary.strong.color),
            })
            .into()
    }

    /// Game settings content
    fn view_game_settings_content(&self) -> Element<'_, Message> {
        column![
            row![
                column![
                    text("Memory Allocation").size(16).font(Font {
                        weight: iced::font::Weight::Semibold,
                        ..Default::default()
                    }),
                    text("Limit the maximum RAM for Minecraft")
                        .size(12)
                        .style(|t: &Theme| text::Style {
                            color: Some(t.extended_palette().secondary.base.text)
                        }),
                ],
                Space::new().width(Length::Fill),
                // RAM
                container(
                    text(format!("{} MB", self.allocated_ram))
                        .size(14)
                        .font(Font {
                            weight: iced::font::Weight::Bold,
                            ..Default::default()
                        })
                )
                .padding([6, 12])
                .style(|theme: &Theme| container::Style {
                    background: Some(Background::Color(
                        theme.extended_palette().primary.weak.color
                    )),
                    border: Border {
                        radius: 8.0.into(),
                        ..Default::default()
                    },
                    text_color: Some(theme.extended_palette().primary.strong.text),
                    ..Default::default()
                })
            ]
            .align_y(Alignment::Center),
            Space::new().height(15),
            slider(1024..=16384, self.allocated_ram, Message::RamChanged),
            Space::new().height(25),
            container(Space::new())
                .width(Length::Fill)
                .height(1)
                .style(|theme: &Theme| container::Style {
                    background: Some(Background::Color(
                        theme.extended_palette().background.strong.color
                    )),
                    ..Default::default()
                }),
            Space::new().height(25),
            // Java Path
            text("Java Runtime Environment").size(16).font(Font {
                weight: iced::font::Weight::Semibold,
                ..Default::default()
            }),
            Space::new().height(10),
            row![
                text_input("Select Java executable path...", &self.java_path)
                    .on_input(Message::JavaPathChanged)
                    .padding(12),
                Space::new().width(10),
                button(text("Browse").size(14))
                    .padding([12, 20])
                    .style(button::primary),
            ]
        ]
        .into()
    }

    /// Launcher settings content
    fn view_launcher_settings_content(&self) -> Element<'_, Message> {
        let languages = vec!["English".to_string(), "Vietnamese".to_string()];

        column![
            // Language
            self.view_setting_row(
                "Language",
                "Select your preferred language",
                pick_list(
                    languages,
                    Some(&self.selected_language),
                    Message::LanguageChanged
                )
                .width(Length::Fixed(160.0))
            ),
            Space::new().height(20),
            // Theme
            self.view_setting_row(
                "Theme",
                "Switch between Light and Dark mode",
                pick_list(
                    vec![Theme::TokyoNight, Theme::TokyoNightLight],
                    Some(&self.current_theme),
                    Message::ThemeChanged
                )
                .width(Length::Fixed(160.0))
            ),
        ]
        .into()
    }

    /// Creates a setting row (label left, control right)
    fn view_setting_row<'a>(
        &self,
        title: &'a str,
        subtitle: &'a str,
        control: impl Into<Element<'a, Message>>,
    ) -> Element<'a, Message> {
        row![
            column![
                text(title).size(16).font(Font {
                    weight: iced::font::Weight::Semibold,
                    ..Default::default()
                }),
                text(subtitle).size(12).style(|t: &Theme| text::Style {
                    color: Some(t.extended_palette().secondary.base.text)
                }),
            ],
            Space::new().width(Length::Fill),
            control.into()
        ]
        .align_y(Alignment::Center)
        .into()
    }

    /// Footer content
    fn view_footer_section(&self) -> Element<'_, Message> {
        column![
            text("Minecraft Launcher v1.0.0")
                .size(12)
                .style(|theme: &Theme| text::Style {
                    color: Some(theme.extended_palette().secondary.weak.text)
                }),
            Space::new().height(10),
            row![
                button(text("Check for Updates").size(12))
                    .on_press(Message::CheckForUpdates)
                    .padding([8, 16])
                    .style(button::secondary),
                Space::new().width(10),
                button(text("GitHub Repo").size(12))
                    .on_press(Message::OpenGithub)
                    .padding([8, 16])
                    .style(button::text),
            ]
            .align_y(Alignment::Center)
        ]
        .align_x(Alignment::Center)
        .into()
    }
}
