// Copyright 2026 MineHighVN, AXLauncher contributors
// SPDX-License-Identifier: Apache-2.0

use crate::common::app_button::AppButton;
use crate::common::app_input::AppInput;
use crate::common::app_text::{AppText, TextVariant};
use crate::theme::{CURRENT_PALETTE, ThemePalette};
use iced::Element;
use iced::widget::Space;

pub struct AppUI;

impl AppUI {
    fn palette() -> ThemePalette {
        *CURRENT_PALETTE
            .read()
            .expect("Failed to read global palette")
    }

    pub fn button<'a, M: Clone + 'a>(label: &'a str) -> AppButton<'a, M> {
        AppButton::new(Self::palette(), label)
    }

    #[allow(unused)]
    pub fn input<'a, M: Clone + 'a>(placeholder: &'a str, value: &'a str) -> AppInput<'a, M> {
        AppInput::new(Self::palette(), placeholder, value)
    }

    #[allow(unused)]
    pub fn card<'a, M: Clone + 'a>(content: impl Into<Element<'a, M>>) -> Element<'a, M> {
        let p = Self::palette();
        iced::widget::container(content)
            .padding(20)
            .style(move |_| iced::widget::container::Style {
                background: Some(p.bg_card.into()),
                border: iced::Border {
                    radius: 12.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .into()
    }

    #[allow(unused)]
    pub fn h1<'a>(content: &'a str) -> AppText<'a> {
        AppText::new(content, TextVariant::H1)
    }

    #[allow(unused)]
    pub fn h2<'a>(content: &'a str) -> AppText<'a> {
        AppText::new(content, TextVariant::H2)
    }

    #[allow(unused)]
    pub fn h3<'a>(content: &'a str) -> AppText<'a> {
        AppText::new(content, TextVariant::H3)
    }

    #[allow(unused)]
    pub fn text<'a>(content: &'a str) -> AppText<'a> {
        AppText::new(content, TextVariant::Normal)
    }

    #[allow(unused)]
    pub fn small<'a>(content: &'a str) -> AppText<'a> {
        AppText::new(content, TextVariant::Small)
    }

    #[allow(unused)]
    pub fn smallest<'a>(content: &'a str) -> AppText<'a> {
        AppText::new(content, TextVariant::Smallest)
    }

    #[allow(unused)]
    pub fn caption<'a, M: 'a>(content: &'a str) -> Element<'a, M> {
        AppText::new(content, TextVariant::Small)
            .caption()
            .build(&Self::palette())
    }

    #[allow(unused)]
    pub fn divider<'a, M: 'a>() -> Element<'a, M> {
        let p = Self::palette();
        iced::widget::container(Space::new().height(1.0))
            .width(iced::Length::Fill)
            .style(move |_| iced::widget::container::Style {
                background: Some(p.text_subtle.scale_alpha(0.1).into()),
                ..Default::default()
            })
            .into()
    }
}
