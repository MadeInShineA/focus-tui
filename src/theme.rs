use ratatui::style::{Color, Style};

use crate::utils::CountdownType;

pub struct Theme {
    pub background_style: Style,
    pub border_style: Style,
    pub text_color: Color,
    pub work_accent_color: Color,
    pub break_accent_color: Color,
}

impl Theme {
    pub fn catppuccin_mocha() -> Self {
        Self {
            background_style: Style::default().bg(Color::Rgb(30, 30, 46)),
            border_style: Style::default().fg(Color::Rgb(205, 214, 244)),
            text_color: Color::Rgb(205, 214, 244),
            work_accent_color: Color::Rgb(166, 227, 161),
            break_accent_color: Color::Rgb(137, 180, 250),
        }
    }

    pub fn catppuccin_latte() -> Self {
        Self {
            background_style: Style::default().bg(Color::Rgb(239, 241, 245)),
            border_style: Style::default().fg(Color::Rgb(76, 79, 105)),
            text_color: Color::Rgb(76, 79, 105),
            work_accent_color: Color::Rgb(64, 160, 43),
            break_accent_color: Color::Rgb(30, 102, 245),
        }
    }

    pub fn gauge_style(&self, countdown_type: &CountdownType) -> Style {
        Style::new()
            .fg(match countdown_type {
                CountdownType::Work => self.work_accent_color,
                CountdownType::Break => self.break_accent_color,
            })
            .bg(self.background_style.bg.unwrap_or(Color::Reset))
    }

    pub fn countdown_color(&self, countdown_type: &CountdownType) -> Color {
        match countdown_type {
            CountdownType::Work => self.work_accent_color,
            CountdownType::Break => self.break_accent_color,
        }
    }

    pub fn text_style(&self) -> Style {
        Style::default().fg(self.text_color)
    }
}
