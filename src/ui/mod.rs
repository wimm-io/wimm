use ratatui::Frame;
use ratatui::crossterm::event;
use thiserror::Error;

use crate::storage::{self, Db};
use crate::types::AppState;

mod app;
mod events;
mod help_panel;
mod input_bar;
mod layout;
mod task_list;

use app::App;
use events::EventHandler;
use help_panel::HelpPanel;
use input_bar::InputBar;
use layout::LayoutManager;
use task_list::TaskList;

pub struct Ui<D: Db> {
    app: App<D>,
    task_list: TaskList,
    input_bar: InputBar,
    help_panel: HelpPanel,
    layout_manager: LayoutManager,
    event_handler: EventHandler,
}

impl<D: Db> Ui<D> {
    pub fn new(app_state: AppState<D>) -> Self {
        Self {
            app: App::new(app_state),
            task_list: TaskList::new(),
            input_bar: InputBar::new(),
            help_panel: HelpPanel::new(),
            layout_manager: LayoutManager::new(),
            event_handler: EventHandler::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), UiError> {
        let mut terminal = ratatui::init();

        while !self.app.state.should_quit {
            terminal.draw(|f| self.draw(f))?;
            let event = event::read()?;
            self.event_handler
                .handle_event(event, &mut self.app, &mut self.task_list);
        }

        ratatui::restore();
        Ok(())
    }

    fn draw(&mut self, f: &mut Frame) {
        let layout = self
            .layout_manager
            .calculate_main_layout(f.area(), &self.app.state);

        // Render title
        self.render_title(f, layout.title);

        // Render main task list
        self.task_list.render(f, layout.main, &self.app.state);

        // Render status bar
        self.render_status(f, layout.status);

        // Render input bar
        self.input_bar.render(f, layout.input, &self.app);

        // Render help panel if visible
        if let Some(help_area) = layout.help {
            self.help_panel.render(f, help_area);
        }
    }

    fn render_title(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        use ratatui::{layout::Alignment, widgets::Paragraph};

        let title = Paragraph::new("Wimm Task Manager - Press 'q' to quit, 'h' for help")
            .alignment(Alignment::Center);
        f.render_widget(title, area);
    }

    fn render_status(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        use ratatui::{layout::Alignment, widgets::Paragraph};

        let mode_text = match self.app.state.mode {
            crate::types::Mode::Normal => "NORMAL",
            crate::types::Mode::Insert => "INSERT",
        };

        let status = format!("Mode: {mode_text}");
        let status_paragraph = Paragraph::new(status).alignment(Alignment::Left);
        f.render_widget(status_paragraph, area);
    }
}

#[derive(Debug, Error)]
pub enum UiError {
    #[error("Terminal IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("DB error: {0}")]
    DbError(#[from] storage::DbError),
}
