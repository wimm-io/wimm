use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph, Wrap},
};

pub struct HelpPanel;

impl HelpPanel {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, f: &mut Frame, area: Rect) {
        let help_text = self.create_help_content();

        let help_paragraph = Paragraph::new(help_text)
            .block(
                Block::bordered().title(" Help ").title_style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
            )
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(Color::White));

        f.render_widget(help_paragraph, area);
    }

    fn create_help_content(&self) -> Vec<Line> {
        vec![
            Line::from(""),
            Line::from(vec![Span::styled(
                "Normal Mode",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from("  j/k     - Move up/down"),
            Line::from("  g/G     - Go to first/last"),
            Line::from("  x       - Toggle completion"),
            Line::from("  D       - Delete task"),
            Line::from("  i       - Insert mode"),
            Line::from("  h       - Toggle help"),
            Line::from("  q       - Quit"),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Insert Mode",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from("  Type    - Add to task title"),
            Line::from("  Enter   - Create task"),
            Line::from("  Backsp  - Delete character"),
            Line::from("  Esc     - Return to Normal"),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Tips",
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from("• Tasks auto-save to database"),
            Line::from("• Navigate with j/k like Vim"),
            Line::from("• Press h to hide this panel"),
        ]
    }
}

impl Default for HelpPanel {
    fn default() -> Self {
        Self::new()
    }
}
