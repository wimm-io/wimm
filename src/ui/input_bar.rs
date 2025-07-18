use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    widgets::Paragraph,
};

use crate::storage::Db;
use crate::types::Mode;
use crate::ui::app::App;

pub struct InputBar;

impl InputBar {
    pub fn new() -> Self {
        Self
    }

    pub fn render<D: Db>(&self, f: &mut Frame, area: Rect, app: &App<D>) {
        match app.state.mode {
            Mode::Insert => {
                let input_text = format!("> {}", &app.state.input_buffer);
                let input_paragraph = Paragraph::new(input_text).alignment(Alignment::Left);
                f.render_widget(input_paragraph, area);
            }
            Mode::Normal => {
                // Show error messages or keep empty in normal mode
                if let Some(ref message) = app.message {
                    let error_paragraph =
                        Paragraph::new(message.as_str()).alignment(Alignment::Left);
                    f.render_widget(error_paragraph, area);
                }
            }
        }
    }
}

impl Default for InputBar {
    fn default() -> Self {
        Self::new()
    }
}
