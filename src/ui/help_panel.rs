use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

pub struct HelpPanel;

impl HelpPanel {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, f: &mut Frame, area: Rect) {
        // Clear the background area to create floating effect
        f.render_widget(Clear, area);

        let help_text = self.create_help_content();

        let help_paragraph = Paragraph::new(help_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Help ")
                    .title_style(
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    )
                    .border_style(Style::default().fg(Color::Cyan))
                    .style(Style::default().bg(Color::DarkGray)),
            )
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(Color::White).bg(Color::DarkGray));

        f.render_widget(help_paragraph, area);
    }

    fn create_help_content(&self) -> Vec<Line> {
        vec![
            Line::from(""),
            Line::from(vec![Span::styled(
                "📋 Normal Mode",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from("  j/k     - Move up/down"),
            Line::from("  g/G     - Go to first/last"),
            Line::from("  !       - Toggle completion"),
            Line::from("  x       - Toggle selection"),
            Line::from("  D       - Delete task"),
            Line::from("  o       - Open new task below"),
            Line::from("  O       - Open new task above"),
            Line::from("  h       - Toggle help"),
            Line::from("  q       - Quit"),
            Line::from(""),
            Line::from(vec![Span::styled(
                "✏️  Insert Mode",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from("  Type    - Edit current field in-place"),
            Line::from("  Tab     - Next field (Title → Description)"),
            Line::from("  S+Tab   - Previous field (Description → Title)"),
            Line::from("  Enter   - Save task & return to Normal"),
            Line::from("  Backsp  - Delete character"),
            Line::from("  Esc     - Cancel & return to Normal"),
            Line::from(""),
            Line::from("Fields are highlighted in yellow when editing."),
            Line::from(""),
        ]
    }
}

impl Default for HelpPanel {
    fn default() -> Self {
        Self::new()
    }
}
