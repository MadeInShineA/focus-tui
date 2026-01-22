use ratatui::layout::{Constraint, Flex, Layout, Rect};

pub const DEFAULT_WORK_DURATION_MINUTES: u64 = 45;
pub const DEFAULT_BREAK_DURATION_MINUTES: u64 = 10;

pub enum CountdownType {
    Work,
    Break,
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
