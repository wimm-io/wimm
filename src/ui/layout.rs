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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::MemoryStorage;
    use crate::types::AppState;
    use std::collections::HashMap;

    fn create_test_app_state() -> AppState<MemoryStorage> {
        let store = MemoryStorage::new(HashMap::new());
        AppState::new(store)
    }

    #[test]
    fn test_layout_manager_new() {
        let _manager = LayoutManager::new();
        // Test passes if creation succeeds without panic
    }

    #[test]
    fn test_layout_manager_default() {
        let _manager = LayoutManager;
        // Test passes if creation succeeds without panic
    }

    #[test]
    fn test_calculate_main_layout_without_help() {
        let manager = LayoutManager::new();
        let app_state = create_test_app_state();
        let area = Rect::new(0, 0, 80, 24);

        let layout = manager.calculate_main_layout(area, &app_state);

        assert_eq!(layout.title.x, 0);
        assert_eq!(layout.title.y, 0);
        assert_eq!(layout.title.width, 80);
        assert_eq!(layout.title.height, 1);

        assert_eq!(layout.main.x, 0);
        assert_eq!(layout.main.y, 1);
        assert_eq!(layout.main.width, 80);
        assert_eq!(layout.main.height, 22);

        assert_eq!(layout.status.x, 0);
        assert_eq!(layout.status.y, 23);
        assert_eq!(layout.status.width, 80);
        assert_eq!(layout.status.height, 1);

        assert!(layout.help.is_none());
    }

    #[test]
    fn test_calculate_main_layout_with_help() {
        let manager = LayoutManager::new();
        let mut app_state = create_test_app_state();
        app_state.show_help = true;
        let area = Rect::new(0, 0, 80, 24);

        let layout = manager.calculate_main_layout(area, &app_state);

        assert_eq!(layout.title.height, 1);
        assert_eq!(layout.main.height, 22);
        assert_eq!(layout.status.height, 1);

        assert!(layout.help.is_some());
        let help_area = layout.help.unwrap();
        assert!(help_area.width <= 50);
        assert!(help_area.height <= 20);
    }

    #[test]
    fn test_calculate_main_layout_small_area() {
        let manager = LayoutManager::new();
        let app_state = create_test_app_state();
        let area = Rect::new(0, 0, 20, 10);

        let layout = manager.calculate_main_layout(area, &app_state);

        assert_eq!(layout.title.width, 20);
        assert_eq!(layout.title.height, 1);
        assert_eq!(layout.main.width, 20);
        assert_eq!(layout.main.height, 8);
        assert_eq!(layout.status.width, 20);
        assert_eq!(layout.status.height, 1);
        assert!(layout.help.is_none());
    }

    #[test]
    fn test_calculate_main_layout_offset_area() {
        let manager = LayoutManager::new();
        let app_state = create_test_app_state();
        let area = Rect::new(10, 5, 60, 20);

        let layout = manager.calculate_main_layout(area, &app_state);

        assert_eq!(layout.title.x, 10);
        assert_eq!(layout.title.y, 5);
        assert_eq!(layout.title.width, 60);

        assert_eq!(layout.main.x, 10);
        assert_eq!(layout.main.y, 6);
        assert_eq!(layout.main.width, 60);

        assert_eq!(layout.status.x, 10);
        assert_eq!(layout.status.y, 24);
        assert_eq!(layout.status.width, 60);
    }

    #[test]
    fn test_calculate_floating_help_normal_area() {
        let manager = LayoutManager::new();
        let area = Rect::new(0, 0, 80, 24);

        let help_area = manager.calculate_floating_help(area);

        assert_eq!(help_area.width, 50);
        assert_eq!(help_area.height, 20);
        assert_eq!(help_area.x, 15); // (80 - 50) / 2
        assert_eq!(help_area.y, 2); // (24 - 20) / 2
    }

    #[test]
    fn test_calculate_floating_help_small_area() {
        let manager = LayoutManager::new();
        let area = Rect::new(0, 0, 30, 10);

        let help_area = manager.calculate_floating_help(area);

        assert_eq!(help_area.width, 26); // 30 - 4 (minimum padding)
        assert_eq!(help_area.height, 6); // 10 - 4 (minimum padding)
        assert_eq!(help_area.x, 2); // (30 - 26) / 2
        assert_eq!(help_area.y, 2); // (10 - 6) / 2
    }

    #[test]
    fn test_calculate_floating_help_very_small_area() {
        let manager = LayoutManager::new();
        let area = Rect::new(0, 0, 10, 6);

        let help_area = manager.calculate_floating_help(area);

        assert_eq!(help_area.width, 6); // 10 - 4 (minimum padding)
        assert_eq!(help_area.height, 2); // 6 - 4 (minimum padding)
        assert_eq!(help_area.x, 2); // (10 - 6) / 2
        assert_eq!(help_area.y, 2); // (6 - 2) / 2
    }

    #[test]
    fn test_calculate_floating_help_offset_area() {
        let manager = LayoutManager::new();
        let area = Rect::new(20, 10, 80, 24);

        let help_area = manager.calculate_floating_help(area);

        assert_eq!(help_area.width, 50);
        assert_eq!(help_area.height, 20);
        assert_eq!(help_area.x, 35); // 20 + (80 - 50) / 2
        assert_eq!(help_area.y, 12); // 10 + (24 - 20) / 2
    }

    #[test]
    fn test_calculate_floating_help_exact_size_area() {
        let manager = LayoutManager::new();
        let area = Rect::new(0, 0, 54, 24); // exactly width for help + padding

        let help_area = manager.calculate_floating_help(area);

        assert_eq!(help_area.width, 50);
        assert_eq!(help_area.height, 20);
        assert_eq!(help_area.x, 2); // (54 - 50) / 2
        assert_eq!(help_area.y, 2); // (24 - 20) / 2
    }

    #[test]
    fn test_main_layout_struct_fields() {
        let title = Rect::new(0, 0, 80, 1);
        let main = Rect::new(0, 1, 80, 22);
        let status = Rect::new(0, 23, 80, 1);
        let help = Some(Rect::new(15, 2, 50, 20));

        let layout = MainLayout {
            title,
            main,
            status,
            help,
        };

        assert_eq!(layout.title, title);
        assert_eq!(layout.main, main);
        assert_eq!(layout.status, status);
        assert_eq!(layout.help, help);
    }

    #[test]
    fn test_main_layout_without_help() {
        let title = Rect::new(0, 0, 80, 1);
        let main = Rect::new(0, 1, 80, 22);
        let status = Rect::new(0, 23, 80, 1);

        let layout = MainLayout {
            title,
            main,
            status,
            help: None,
        };

        assert_eq!(layout.title, title);
        assert_eq!(layout.main, main);
        assert_eq!(layout.status, status);
        assert!(layout.help.is_none());
    }

    #[test]
    fn test_layout_consistency_different_sizes() {
        let manager = LayoutManager::new();
        let app_state = create_test_app_state();

        let sizes = vec![(20, 10), (40, 15), (80, 24), (120, 40), (200, 60)];

        for (width, height) in sizes {
            let area = Rect::new(0, 0, width, height);
            let layout = manager.calculate_main_layout(area, &app_state);

            // Title should always be 1 high
            assert_eq!(layout.title.height, 1);

            // Status should always be 1 high
            assert_eq!(layout.status.height, 1);

            // Main should fill remaining space
            assert_eq!(layout.main.height, height.saturating_sub(2));

            // All areas should have the same width as the parent
            assert_eq!(layout.title.width, width);
            assert_eq!(layout.main.width, width);
            assert_eq!(layout.status.width, width);

            // Areas should be stacked vertically
            assert_eq!(layout.title.y, 0);
            assert_eq!(layout.main.y, 1);
            assert_eq!(layout.status.y, height.saturating_sub(1));
        }
    }
}
