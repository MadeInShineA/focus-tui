use ratatui::{
    Frame,
    crossterm::event::{Event, KeyCode, KeyEvent},
    layout::{Alignment, Constraint, Layout, Rect},
    style::Style,
    text::Text,
    widgets::{Block, Borders, Clear, Paragraph},
};
use uuid::Uuid;

use crate::{
    app::Action,
    task::{Task, TaskStatus},
    theme::Theme,
    utils::popup_area,
};

enum SelectedField {
    Title,
    Status,
}

pub struct EditTaskPopup {
    task_opened_on_idx: usize,
    task_opened_on_uuid: Uuid,
    current_title: String,
    current_status: TaskStatus,
    selected_field: SelectedField,
}

impl EditTaskPopup {
    pub fn new(task_opened_on_idx: usize, task: Task) -> Self {
        EditTaskPopup {
            task_opened_on_idx,
            task_opened_on_uuid: task.uuid,
            current_title: task.title,
            current_status: task.status,
            selected_field: SelectedField::Title,
        }
    }

    fn handle_key_event(&mut self, key_event: &KeyEvent) -> Option<Action> {
        match key_event.code {
            // KeyCode::Esc => Some(Action::ClosePopup),
            KeyCode::Esc => Some(Action::OpenTaskList {
                selected_task_idx: self.task_opened_on_idx,
            }),
            KeyCode::Enter => {
                if !self.current_title.trim().is_empty() {
                    Some(Action::EditTask {
                        return_to: self.task_opened_on_idx,
                        task: Task {
                            uuid: self.task_opened_on_uuid,
                            title: self.current_title.clone(),
                            status: self.current_status.clone(),
                        },
                    })
                } else {
                    Some(Action::ShowError("Missing task title".to_string()))
                }
            }
            KeyCode::Tab | KeyCode::Left | KeyCode::Right => {
                match self.selected_field {
                    SelectedField::Title => self.selected_field = SelectedField::Status,
                    SelectedField::Status => self.selected_field = SelectedField::Title,
                }
                None
            }
            _ => {
                match self.selected_field {
                    SelectedField::Title => match key_event.code {
                        KeyCode::Backspace => {
                            self.current_title.pop();
                        }
                        KeyCode::Char(c) => {
                            self.current_title.push(c);
                        }
                        _ => {}
                    },
                    SelectedField::Status => match key_event.code {
                        KeyCode::Up => self.current_status = self.current_status.previous(),
                        KeyCode::Down => self.current_status = self.current_status.next(),
                        _ => {}
                    },
                }
                None
            }
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, area: Rect, theme: &Theme) {
        let block = Block::bordered()
            .border_style(theme.border_style)
            .style(theme.background_style);

        let popup_area: Rect = popup_area(area, 30, 30);
        frame.render_widget(Clear, popup_area);
        let inner_area: Rect = block.inner(popup_area);
        let inner_layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(vec![
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Length(3),
                Constraint::Fill(1),
            ])
            .split(inner_area);

        let title_text: Text = Text::styled("Edit task popup", theme.text_style()).centered();
        let title_paragraphe: Paragraph = Paragraph::new(title_text).centered();

        let task_chunks = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Percentage(45),
            Constraint::Percentage(45),
            Constraint::Fill(1),
        ])
        .split(inner_layout[2]);

        let mut title_block = Block::default()
            .title("Title")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL);
        let mut status_block = Block::default()
            .title("Status")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL);

        let active_style = Style::default().reversed();

        match self.selected_field {
            SelectedField::Title => title_block = title_block.style(active_style),
            SelectedField::Status => status_block = status_block.style(active_style),
        }

        let task_title_paragraph: Paragraph = Paragraph::new(self.current_title.clone())
            .block(title_block)
            .centered();
        let status_paragraph: Paragraph = Paragraph::new(self.current_status.to_string())
            .block(status_block)
            .centered();

        frame.render_widget(block, popup_area);
        frame.render_widget(title_paragraphe, inner_layout[0]);
        frame.render_widget(task_title_paragraph, task_chunks[1]);
        frame.render_widget(status_paragraph, task_chunks[2]);
    }

    pub fn handle_event(&mut self, event: &Event) -> Option<Action> {
        match event {
            Event::Key(key_event) => self.handle_key_event(key_event),

            _ => None,
        }
    }
}
