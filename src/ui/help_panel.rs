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
            Line::from("  i       - Edit current task"),
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
            Line::from("  Tab     - Next field (Title → Description → Due → Defer)"),
            Line::from("  S+Tab   - Previous field"),
            Line::from("  Enter   - Save task & return to Normal"),
            Line::from("  Backsp  - Delete character"),
            Line::from("  Esc     - Cancel & return to Normal"),
            Line::from(""),
            Line::from("Fields are highlighted in yellow when editing."),
            Line::from(""),
            Line::from(vec![Span::styled(
                "📅 Date Formats",
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from("  Relative: 2d, 1w, 3h, 30m"),
            Line::from("  Keywords: today, tomorrow, yesterday"),
            Line::from("  Weekdays: friday, next monday"),
            Line::from("  Absolute: 2024-12-25, 12-25"),
            Line::from("  (empty)  - Clear date"),
            Line::from(""),
            Line::from("  Due dates default to 5pm, defer dates to 8am"),
            Line::from(""),
            Line::from(vec![Span::styled(
                "🎨 Visual Highlights",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "  Red text",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
                Span::raw(" - Due today or overdue"),
            ]),
            Line::from(vec![
                Span::styled(
                    "  Yellow text",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" - Due within 24 hours"),
            ]),
            Line::from(vec![
                Span::styled("  Dark gray text", Style::default().fg(Color::DarkGray)),
                Span::raw(" - Deferred (hidden until defer time)"),
            ]),
            Line::from(""),
        ]
    }
}

impl Default for HelpPanel {
    fn default() -> Self {
        Self::new()
    }
}
