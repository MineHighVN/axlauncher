// Copyright 2026 MineHighVN, AXLauncher contributors
// SPDX-License-Identifier: Apache-2.0

#![allow(dead_code)]
use crate::theme::ThemePalette;
use iced::widget::{button, text};
use iced::{Element, Length, Padding};

pub struct AppButton<'a, Message> {
    palette: ThemePalette,
    label: &'a str,
    on_press: Option<Message>,
    padding: Padding,
    width: Length,
}

impl<'a, Message> AppButton<'a, Message>
where
    Message: Clone + 'a,
{
    pub fn new(palette: ThemePalette, label: &'a str) -> Self {
        Self {
            palette,
            label,
            on_press: None,
            padding: [10, 20].into(),
            width: Length::Shrink,
        }
    }

    pub fn on_press(mut self, msg: Message) -> Self {
        self.on_press = Some(msg);
        self
    }

    pub fn padding(mut self, p: impl Into<Padding>) -> Self {
        self.padding = p.into();
        self
    }

    pub fn width(mut self, w: impl Into<Length>) -> Self {
        self.width = w.into();
        self
    }

    pub fn build(self) -> Element<'a, Message> {
        let palette = self.palette;
        let mut btn = button(text(self.label).size(14))
            .padding(self.padding)
            .width(self.width)
            .style(move |_theme, status| {
                let base_color = palette.primary;

                match status {
                    button::Status::Hovered => button::Style {
                        // When hovered
                        background: Some(iced::Background::Color(iced::Color {
                            r: (base_color.r + 0.1).min(1.0),
                            g: (base_color.g + 0.1).min(1.0),
                            b: (base_color.b + 0.1).min(1.0),
                            ..base_color
                        })),
                        text_color: iced::Color::WHITE,
                        border: iced::Border {
                            radius: 8.0.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    button::Status::Pressed => button::Style {
                        // When pressed
                        background: Some(iced::Background::Color(iced::Color {
                            r: (base_color.r - 0.1).max(0.0),
                            g: (base_color.g - 0.1).max(0.0),
                            b: (base_color.b - 0.1).max(0.0),
                            ..base_color
                        })),
                        text_color: iced::Color::WHITE,
                        border: iced::Border {
                            radius: 8.0.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    _ => button::Style {
                        // Normal
                        background: Some(base_color.into()),
                        text_color: iced::Color::WHITE,
                        border: iced::Border {
                            radius: 8.0.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                }
            });

        if let Some(msg) = self.on_press {
            btn = btn.on_press(msg);
        }

        btn.into()
    }
}
