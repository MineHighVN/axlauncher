// Copyright 2026 MineHighVN, AXLauncher contributors
// SPDX-License-Identifier: Apache-2.0

use iced::{
    Alignment, Background, Border, Element, Font, Length, Padding, Theme,
    widget::{Space, button, column, container, row, text},
};

use crate::state::Page;

#[derive(Debug, Clone)]
pub enum Message {
    PageSelected(Page),
}

fn sidebar_container_style(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();
    container::Style {
        background: Some(Background::Color(palette.background.weak.color)),
        border: Border {
            width: 1.0,
            color: palette.background.strong.color,
            radius: 12.0.into(),
        },
        ..Default::default()
    }
}

fn nav_button_style(theme: &Theme, status: button::Status, is_active: bool) -> button::Style {
    let palette = theme.extended_palette();

    let base_style = button::Style {
        text_color: palette.background.base.text,
        border: Border {
            radius: 8.0.into(),
            ..Default::default()
        },
        background: None,
        ..Default::default()
    };

    if is_active {
        return button::Style {
            background: Some(Background::Color(palette.primary.base.color)),
            text_color: palette.primary.strong.text,
            ..base_style
        };
    }

    match status {
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(palette.background.strong.color)),
            text_color: palette.background.strong.text,
            border: Border {
                radius: 8.0.into(),
                ..Default::default()
            },
            ..base_style
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(palette.primary.weak.color)),

            ..base_style
        },
        _ => base_style,
    }
}

fn nav_item<'a>(
    label: &'a str,
    icon: &'a str,
    page: Page,
    current_page: Page,
) -> Element<'a, Message> {
    let is_active = page == current_page;

    button(
        row![text(icon).size(20), text(label).size(16)]
            .spacing(12)
            .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .padding([10, 15])
    .on_press(Message::PageSelected(page))
    .style(move |theme: &Theme, status| nav_button_style(theme, status, is_active))
    .into()
}

pub fn app_sidebar<'a>(current_page: Page) -> Element<'a, Message> {
    let header = column![
        text("MINECRAFT").size(24).font(Font {
            weight: iced::font::Weight::Bold,
            ..Default::default()
        }),
        text("LAUNCHER")
            .size(12)
            .font(Font {
                weight: iced::font::Weight::Normal,
                ..Default::default()
            })
            .style(|theme: &Theme| text::Style {
                color: Some(theme.extended_palette().secondary.base.color)
            }),
    ]
    .spacing(4)
    .padding(Padding {
        top: 20.0,
        right: 10.0,
        bottom: 40.0,
        left: 10.0,
    });

    let content = column![
        nav_item("Home", "", Page::Home, current_page),
        nav_item("Instances", "", Page::Instances, current_page),
        nav_item("Accounts", "", Page::Accounts, current_page),
    ]
    .spacing(8);

    let footer = column![nav_item("Settings", "", Page::Settings, current_page)];

    container(column![
        header,
        content,
        Space::new().height(Length::Fill),
        footer
    ])
    .width(Length::Fixed(240.0))
    .height(Length::Fill)
    .padding(15)
    .style(sidebar_container_style)
    .into()
}
