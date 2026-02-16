// Copyright 2026 MineHighVN, AXLauncher contributors
// SPDX-License-Identifier: Apache-2.0

#![allow(dead_code)]

use crate::theme::ThemePalette;
use iced::widget::text_input;
use iced::{Background, Border, Element, Length, Padding};

pub struct AppInput<'a, Message> {
    palette: ThemePalette,
    placeholder: &'a str,
    value: &'a str,
    on_input: Option<Box<dyn Fn(String) -> Message + 'a>>,
    padding: Padding,
    width: Length,
}

impl<'a, Message> AppInput<'a, Message> {
    pub fn new(palette: ThemePalette, placeholder: &'a str, value: &'a str) -> Self {
        Self {
            palette,
            placeholder,
            value,
            on_input: None,
            padding: Padding::new(10.0),
            width: Length::Fill,
        }
    }

    pub fn on_input(mut self, f: impl Fn(String) -> Message + 'a) -> Self {
        self.on_input = Some(Box::new(f));
        self
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn build(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let palette = self.palette;

        let mut input = text_input(self.placeholder, self.value)
            .width(self.width)
            .padding(12)
            .style(move |_theme, status| {
                let border_color = match status {
                    text_input::Status::Focused { .. } => palette.primary,
                    text_input::Status::Hovered => palette.text_main,
                    text_input::Status::Disabled => palette.bg_card,
                    _ => palette.bg_main,
                };

                text_input::Style {
                    background: Background::Color(palette.bg_main),
                    border: Border {
                        color: border_color,
                        width: 1.0,
                        radius: 6.0.into(),
                    },
                    icon: palette.text_subtle,
                    placeholder: palette.text_subtle,
                    value: palette.text_main,
                    selection: palette.primary,
                }
            });

        if let Some(on_input) = self.on_input {
            input = input.on_input(on_input);
        }

        input.into()
    }
}
