// Copyright 2026 MineHighVN, AXLauncher contributors
// SPDX-License-Identifier: Apache-2.0

#![allow(dead_code)]
use iced::widget::text;
use iced::{Color, Element, Font};

use crate::theme::ThemePalette;

pub enum TextVariant {
    H1,
    H2,
    H3,
    H4,
    H5,
    Normal,
    Small,
    Smallest,
}

#[derive(Debug, Clone, Copy)]
pub enum TextStatus {
    Normal,
    Caption,
    Danger,
    Success,
}

pub struct AppText<'a> {
    content: &'a str,
    variant: TextVariant,
    status: TextStatus,
    color: Option<Color>,
}

impl<'a> AppText<'a> {
    pub fn new(content: &'a str, variant: TextVariant) -> Self {
        Self {
            content,
            variant,
            status: TextStatus::Normal,
            color: None,
        }
    }

    pub fn danger(mut self) -> Self {
        self.status = TextStatus::Danger;
        self
    }
    pub fn success(mut self) -> Self {
        self.status = TextStatus::Success;
        self
    }
    pub fn caption(mut self) -> Self {
        self.status = TextStatus::Caption;
        self
    }

    pub fn build<Message>(self, palette: &ThemePalette) -> Element<'a, Message> {
        let size = match self.variant {
            TextVariant::H1 => 32,
            TextVariant::H2 => 28,
            TextVariant::H3 => 24,
            TextVariant::Normal => 14,
            TextVariant::Small => 12,
            TextVariant::Smallest => 10,
            _ => 14,
        };

        let final_color = self.color.unwrap_or_else(|| match self.status {
            TextStatus::Normal => palette.text_main,
            TextStatus::Caption => palette.text_subtle,
            TextStatus::Danger => palette.danger,
            TextStatus::Success => palette.success,
        });

        text(self.content)
            .size(size)
            .color(final_color)
            .font(Font {
                weight: if matches!(
                    self.variant,
                    TextVariant::H1 | TextVariant::H2 | TextVariant::H3
                ) {
                    iced::font::Weight::Bold
                } else {
                    iced::font::Weight::Normal
                },
                ..Default::default()
            })
            .into()
    }
}
