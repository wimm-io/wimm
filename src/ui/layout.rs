use crate::storage::Db;
use crate::types::AppState;
use ratatui::layout::{Constraint, Layout, Rect};

pub struct LayoutManager;

impl LayoutManager {
    pub fn new() -> Self {
        Self
    }

    pub fn calculate_main_layout<D: Db>(&self, area: Rect, app_state: &AppState<D>) -> MainLayout {
        let main_layout = Layout::vertical([
            Constraint::Length(1), // Title
            Constraint::Min(1),    // Main content
            Constraint::Length(1), // Status
        ]);

        let [title_area, main_area, status_area] = main_layout.areas(area);

        let help_area = if app_state.show_help {
            Some(self.calculate_floating_help(area))
        } else {
            None
        };

        MainLayout {
            title: title_area,
            main: main_area,
            status: status_area,
            help: help_area,
        }
    }

    fn calculate_floating_help(&self, area: Rect) -> Rect {
        // Create a centered floating panel
        let help_width = 50.min(area.width.saturating_sub(4));
        let help_height = 20.min(area.height.saturating_sub(4));

        let x = (area.width.saturating_sub(help_width)) / 2;
        let y = (area.height.saturating_sub(help_height)) / 2;

        Rect {
            x: area.x + x,
            y: area.y + y,
            width: help_width,
            height: help_height,
        }
    }
}

pub struct MainLayout {
    pub title: Rect,
    pub main: Rect,
    pub status: Rect,
    pub help: Option<Rect>,
}

impl Default for LayoutManager {
    fn default() -> Self {
        Self::new()
    }
}
