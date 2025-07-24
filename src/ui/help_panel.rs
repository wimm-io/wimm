use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
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

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{backend::TestBackend, layout::Rect, Terminal};

    #[test]
    fn test_help_panel_new() {
        let _panel = HelpPanel::new();
        // Just verify it creates successfully
        // Test passes if creation succeeds without panic
    }

    #[test]
    fn test_help_panel_default() {
        let _panel = HelpPanel;
        // Just verify it creates successfully
        // Test passes if creation succeeds without panic
    }

    #[test]
    fn test_create_help_content() {
        let panel = HelpPanel::new();
        let content = panel.create_help_content();

        assert!(!content.is_empty());

        // Check that content contains expected sections
        let content_text = content
            .iter()
            .map(|line| {
                line.spans
                    .iter()
                    .map(|span| span.content.as_ref())
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n");

        assert!(content_text.contains("Normal Mode"));
        assert!(content_text.contains("Insert Mode"));
        assert!(content_text.contains("Date Formats"));
        assert!(content_text.contains("Visual Highlights"));
    }

    #[test]
    fn test_help_content_keybindings() {
        let panel = HelpPanel::new();
        let content = panel.create_help_content();

        let content_text = content
            .iter()
            .map(|line| {
                line.spans
                    .iter()
                    .map(|span| span.content.as_ref())
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n");

        // Check for specific keybindings
        assert!(content_text.contains("j/k"));
        assert!(content_text.contains("g/G"));
        assert!(content_text.contains("!"));
        assert!(content_text.contains("x"));
        assert!(content_text.contains("D"));
        assert!(content_text.contains("o"));
        assert!(content_text.contains("O"));
        assert!(content_text.contains("i"));
        assert!(content_text.contains("h"));
        assert!(content_text.contains("q"));
        assert!(content_text.contains("Tab"));
        assert!(content_text.contains("Enter"));
        assert!(content_text.contains("Backsp"));
        assert!(content_text.contains("Esc"));
    }

    #[test]
    fn test_help_content_date_formats() {
        let panel = HelpPanel::new();
        let content = panel.create_help_content();

        let content_text = content
            .iter()
            .map(|line| {
                line.spans
                    .iter()
                    .map(|span| span.content.as_ref())
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n");

        // Check for date format examples
        assert!(content_text.contains("2d"));
        assert!(content_text.contains("1w"));
        assert!(content_text.contains("3h"));
        assert!(content_text.contains("30m"));
        assert!(content_text.contains("today"));
        assert!(content_text.contains("tomorrow"));
        assert!(content_text.contains("yesterday"));
        assert!(content_text.contains("friday"));
        assert!(content_text.contains("monday"));
        assert!(content_text.contains("2024-12-25"));
        assert!(content_text.contains("12-25"));
    }

    #[test]
    fn test_help_content_visual_highlights() {
        let panel = HelpPanel::new();
        let content = panel.create_help_content();

        let content_text = content
            .iter()
            .map(|line| {
                line.spans
                    .iter()
                    .map(|span| span.content.as_ref())
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n");

        assert!(content_text.contains("Red text"));
        assert!(content_text.contains("Yellow text"));
        assert!(content_text.contains("Dark gray text"));
        assert!(content_text.contains("overdue"));
        assert!(content_text.contains("24 hours"));
        assert!(content_text.contains("Deferred"));
    }

    #[test]
    fn test_help_content_has_proper_structure() {
        let panel = HelpPanel::new();
        let content = panel.create_help_content();

        // Should have multiple lines
        assert!(content.len() > 10);

        // First line should be empty for spacing
        if !content.is_empty() && !content[0].spans.is_empty() {
            assert_eq!(content[0].spans[0].content, "");
        }

        // Should contain styled headers
        let styled_lines: Vec<_> = content
            .iter()
            .filter(|line| {
                line.spans.iter().any(|span| {
                    span.style.fg == Some(Color::Green)
                        || span.style.fg == Some(Color::Blue)
                        || span.style.fg == Some(Color::Magenta)
                        || span.style.fg == Some(Color::Cyan)
                })
            })
            .collect();

        assert!(styled_lines.len() >= 4); // At least 4 section headers
    }

    #[test]
    fn test_help_content_line_structure() {
        let panel = HelpPanel::new();
        let content = panel.create_help_content();

        // Check that some lines have multiple spans (for styled content)
        let multi_span_lines: Vec<_> = content.iter().filter(|line| line.spans.len() > 1).collect();

        assert!(!multi_span_lines.is_empty());

        // Check that some lines are simple text (single span)
        let single_span_lines: Vec<_> = content
            .iter()
            .filter(|line| line.spans.len() == 1)
            .collect();

        assert!(!single_span_lines.is_empty());
    }

    #[test]
    fn test_render_help_panel() {
        let panel = HelpPanel::new();
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
                let area = Rect::new(10, 5, 60, 14);
                panel.render(f, area);
            })
            .unwrap();

        // If we get here without panicking, the render worked
        // Test passes if no panic occurs
    }

    #[test]
    fn test_render_help_panel_small_area() {
        let panel = HelpPanel::new();
        let backend = TestBackend::new(40, 10);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
                let area = Rect::new(0, 0, 40, 10);
                panel.render(f, area);
            })
            .unwrap();

        // Should handle small areas gracefully
        // Test passes if no panic occurs
    }

    #[test]
    fn test_render_help_panel_full_screen() {
        let panel = HelpPanel::new();
        let backend = TestBackend::new(120, 40);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
                let area = f.area();
                panel.render(f, area);
            })
            .unwrap();

        // Should handle full screen rendering
        // Test passes if no panic occurs
    }
}
