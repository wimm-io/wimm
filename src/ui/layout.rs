use crate::storage::Db;
use crate::types::AppState;
use ratatui::layout::{Constraint, Layout, Rect};

pub struct LayoutManager;

impl LayoutManager {
    pub fn new() -> Self {
        Self
    }

    pub fn calculate_main_layout<D: Db>(&self, area: Rect, app_state: &AppState<D>) -> MainLayout {
        if app_state.show_help {
            self.calculate_with_help(area)
        } else {
            self.calculate_without_help(area)
        }
    }

    fn calculate_without_help(&self, area: Rect) -> MainLayout {
        let main_layout = Layout::vertical([
            Constraint::Length(1), // Title
            Constraint::Min(1),    // Main content
            Constraint::Length(1), // Status
            Constraint::Length(1), // Input
        ]);

        let [title_area, main_area, status_area, input_area] = main_layout.areas(area);

        MainLayout {
            title: title_area,
            main: main_area,
            status: status_area,
            input: input_area,
            help: None,
        }
    }

    fn calculate_with_help(&self, area: Rect) -> MainLayout {
        let horizontal_layout = Layout::horizontal([
            Constraint::Percentage(60), // Left side (tasks)
            Constraint::Percentage(40), // Right side (help)
        ]);

        let [left_area, help_area] = horizontal_layout.areas(area);

        let left_layout = Layout::vertical([
            Constraint::Length(1), // Title
            Constraint::Min(1),    // Tasks
            Constraint::Length(1), // Status
            Constraint::Length(1), // Input
        ]);

        let [title_area, main_area, status_area, input_area] = left_layout.areas(left_area);

        MainLayout {
            title: title_area,
            main: main_area,
            status: status_area,
            input: input_area,
            help: Some(help_area),
        }
    }
}

pub struct MainLayout {
    pub title: Rect,
    pub main: Rect,
    pub status: Rect,
    pub input: Rect,
    pub help: Option<Rect>,
}

impl Default for LayoutManager {
    fn default() -> Self {
        Self::new()
    }
}
