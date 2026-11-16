use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent},
    layout::{Constraint, Layout, Rect},
    style::Style,
    text::Text,
    widgets::{Block, Clear, List, ListItem, ListState, Paragraph},
};

use crate::{app::Action, task::Task, theme::Theme, utils::popup_area};

pub struct TaskListPopup {
    list_state: ListState,
}

impl TaskListPopup {
    pub fn new(selected_task_idx: usize) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(selected_task_idx));

        TaskListPopup { list_state }
    }

    fn handle_key_event(&mut self, key_event: &KeyEvent, tasks: &[Task]) -> Option<Action> {
        match key_event.code {
            KeyCode::Char('t') | KeyCode::Esc => return Some(Action::ClosePopup),
            KeyCode::Char('a') => {
                return Some(Action::OpenAddTask {
                    return_to: (self.list_state.selected().unwrap_or(0)),
                });
            }

            KeyCode::Char('d') => {
                if let Some(selected_task) = self.list_state.selected().and_then(|i| tasks.get(i)) {
                    return Some(Action::DeleteTask {
                        task_uuid: selected_task.uuid,
                    });
                }
            }
            KeyCode::Char('e') => {
                if let Some((selected_task_idx, selected_task)) = self
                    .list_state
                    .selected()
                    .and_then(|i| tasks.get(i).map(|task| (i, task)))
                {
                    return Some(Action::OpenEditTask {
                        return_to: selected_task_idx,
                        selected_task: selected_task.clone(),
                    });
                }
            }

            KeyCode::Up => self.list_state.select_previous(),
            KeyCode::Down => self.list_state.select_next(),
            KeyCode::Tab => {
                if let Some(selected_task) = self.list_state.selected().and_then(|i| tasks.get(i)) {
                    return Some(Action::CycleTaskStatus {
                        task_uuid: selected_task.uuid,
                    });
                }
            }
            _ => {}
        }
        None
    }

    pub fn draw(
        &mut self,
        frame: &mut ratatui::Frame,
        area: ratatui::prelude::Rect,
        theme: &Theme,
        tasks: &[Task],
    ) {
        if tasks.is_empty() {
            self.list_state.select(None);
        } else {
            let i = self.list_state.selected().unwrap_or(0).min(tasks.len() - 1);
            self.list_state.select(Some(i));
        }

        let block = Block::bordered()
            .border_style(theme.border_style)
            .style(theme.background_style);

        let popup_area: Rect = popup_area(area, 60, 60);
        frame.render_widget(Clear, popup_area);
        let inner_area: Rect = block.inner(popup_area);
        let inner_layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(vec![Constraint::Length(1), Constraint::Fill(1)])
            .split(inner_area);

        let title_text: Text = Text::styled("Task list popup", theme.text_style()).centered();
        let title_paragraphe: Paragraph = Paragraph::new(title_text).centered();

        let task_list: List = List::new(
            tasks
                .iter()
                .map(|task| task.get_list_item().style(theme.text_style()))
                .collect::<Vec<ListItem>>(),
        )
        .highlight_style(Style::default().fg(theme.text_color).bold());

        frame.render_widget(block, popup_area);
        frame.render_widget(title_paragraphe, inner_layout[0]);
        frame.render_stateful_widget(task_list, inner_layout[1], &mut self.list_state);
    }

    pub fn handle_event(&mut self, event: &Event, tasks: &[Task]) -> Option<Action> {
        match event {
            Event::Key(key_event) => self.handle_key_event(key_event, tasks),

            _ => None,
        }
    }
}
