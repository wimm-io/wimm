use ratatui::{
    Frame,
    layout::Rect,
    text::Line,
    widgets::{Block, HighlightSpacing, List, ListItem, ListState, Padding},
};

use crate::storage::Db;
use crate::types::AppState;

pub struct TaskList {
    state: ListState,
}

impl TaskList {
    pub fn new() -> Self {
        Self {
            state: ListState::default(),
        }
    }

    pub fn render<D: Db>(&mut self, f: &mut Frame, area: Rect, app_state: &AppState<D>) {
        // Auto-select first item if nothing is selected and tasks exist
        if !app_state.tasks.is_empty() && self.state.selected().is_none() {
            self.state.select_first();
        }

        let list_items: Vec<ListItem> = app_state
            .tasks
            .iter()
            .map(|task| {
                ListItem::new(Line::from(format!(
                    "[{}] {}",
                    if task.completed { "x" } else { " " },
                    task.title
                )))
            })
            .collect();

        let list = List::new(list_items)
            .block(
                Block::bordered()
                    .padding(Padding::uniform(1))
                    .title(Line::from(format!(" Tasks ({}) ", app_state.tasks.len()))),
            )
            .highlight_symbol("> ")
            .highlight_spacing(HighlightSpacing::Always);

        f.render_stateful_widget(list, area, &mut self.state);
    }

    pub fn select_next(&mut self) {
        self.state.select_next();
    }

    pub fn select_previous(&mut self) {
        self.state.select_previous();
    }

    pub fn select_first(&mut self) {
        self.state.select_first();
    }

    pub fn select_last(&mut self) {
        self.state.select_last();
    }

    pub fn selected_index(&self) -> Option<usize> {
        self.state.selected()
    }

    pub fn move_selection_to_last(&mut self) {
        self.state.select_last();
    }

    pub fn adjust_selection_after_delete(&mut self) {
        self.state.select_previous();
    }
}

impl Default for TaskList {
    fn default() -> Self {
        Self::new()
    }
}
