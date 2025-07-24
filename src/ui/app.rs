use chrono::{DateTime, Datelike, Local, NaiveDate, TimeZone, Weekday};
use std::collections::HashSet;
use std::time::{Duration, SystemTime};

use crate::{
    storage::{Db, DbError},
    types::{AppState, Task},
};
use ratatui::widgets::TableState;
use uuid::Uuid;

pub struct App<D: Db> {
    pub state: AppState<D>,
    pub message: Option<String>,
    pub task_list_state: TableState,
    task_selection: HashSet<usize>,
}

impl<D: Db> App<D> {
    pub fn new(state: AppState<D>) -> Self {
        Self {
            state,
            message: None,
            task_list_state: TableState::default(),
            task_selection: HashSet::default(),
        }
    }

    pub fn add_task(&mut self, title: &str) -> Result<(), DbError> {
        let new_task = self.create_task(title);
        self.state.tasks.push(new_task);
        self.sync_to_storage()
    }

    pub fn toggle_task_completion(&mut self) -> Result<(), DbError> {
        self.apply_to_selection(|t| t.completed = !t.completed);
        self.clear_task_selection();
        Ok(())
    }

    pub fn delete_tasks(&mut self) -> Result<(), DbError> {
        let mut indices: Vec<usize> = self.selection().collect();
        indices.sort();

        for index in indices.iter().rev() {
            if *index < self.state.tasks.len() {
                self.state.tasks.swap_remove(*index);
            }
        }
        self.sync_to_storage()?;
        self.clear_task_selection();
        Ok(())
    }

    pub fn quit(&mut self) {
        self.state.should_quit = true;
    }

    pub fn clear_input_buffer(&mut self) {
        self.state.input_buffer.clear();
    }

    pub fn add_to_input_buffer(&mut self, c: char) {
        self.state.input_buffer.push(c);
    }

    pub fn backspace_input_buffer(&mut self) {
        self.state.input_buffer.pop();
    }

    pub fn set_error_message(&mut self, message: String) {
        self.message = Some(message);
    }

    pub fn clear_error_message(&mut self) {
        self.message = None;
    }

    pub fn get_error_message(&self) -> Option<&String> {
        self.message.as_ref()
    }

    pub fn parse_date_input(&self, input: &str, is_due_date: bool) -> Option<SystemTime> {
        let input = input.trim().to_lowercase();
        if input.is_empty() || input == "-" {
            return None;
        }

        let now = SystemTime::now();
        let local_now = DateTime::<Local>::from(now);

        // Default hour based on date type: due dates at 5pm, defer dates at 8am
        let default_hour = if is_due_date { 17 } else { 8 };

        // Handle simple keywords
        match input.as_str() {
            "today" => return Some(now),
            "tomorrow" => {
                let tomorrow = local_now.date_naive().succ_opt()?;
                let tomorrow_dt = Local
                    .from_local_datetime(&tomorrow.and_hms_opt(default_hour, 0, 0)?)
                    .single()?;
                return Some(tomorrow_dt.into());
            }
            "yesterday" => {
                let yesterday = local_now.date_naive().pred_opt()?;
                let yesterday_dt = Local
                    .from_local_datetime(&yesterday.and_hms_opt(default_hour, 0, 0)?)
                    .single()?;
                return Some(yesterday_dt.into());
            }
            _ => {}
        }

        // Handle weekday names
        if let Some(target_weekday) = self.parse_weekday(&input) {
            let current_weekday = local_now.weekday();
            let days_ahead = (target_weekday.num_days_from_monday() as i64
                - current_weekday.num_days_from_monday() as i64
                + 7)
                % 7;
            let days_ahead = if days_ahead == 0 { 7 } else { days_ahead }; // Next occurrence

            let target_date = local_now.date_naive() + chrono::Duration::days(days_ahead);
            let target_dt = Local
                .from_local_datetime(&target_date.and_hms_opt(default_hour, 0, 0)?)
                .single()?;
            return Some(target_dt.into());
        }

        // Handle "next weekday"
        if let Some(weekday_part) = input.strip_prefix("next ") {
            if let Some(target_weekday) = self.parse_weekday(weekday_part) {
                let current_weekday = local_now.weekday();
                let days_ahead = (target_weekday.num_days_from_monday() as i64
                    - current_weekday.num_days_from_monday() as i64
                    + 14)
                    % 7;
                let days_ahead = if days_ahead == 0 { 7 } else { days_ahead } + 7; // Next week

                let target_date = local_now.date_naive() + chrono::Duration::days(days_ahead);
                let target_dt = Local
                    .from_local_datetime(&target_date.and_hms_opt(default_hour, 0, 0)?)
                    .single()?;
                return Some(target_dt.into());
            }
        }

        // Handle relative dates like "2d", "1w", "3h"
        if let Some(last_char) = input.chars().last() {
            if let Ok(num) = input[..input.len() - 1].parse::<u64>() {
                let duration = match last_char {
                    'd' => Duration::from_secs(num * 24 * 60 * 60),
                    'h' => Duration::from_secs(num * 60 * 60),
                    'm' => Duration::from_secs(num * 60),
                    'w' => Duration::from_secs(num * 7 * 24 * 60 * 60),
                    _ => return None,
                };
                return now.checked_add(duration);
            }
        }

        // Handle YYYY-MM-DD format
        if let Ok(date) = NaiveDate::parse_from_str(&input, "%Y-%m-%d") {
            let dt = Local
                .from_local_datetime(&date.and_hms_opt(default_hour, 0, 0)?)
                .single()?;
            return Some(dt.into());
        }

        // Handle MM-DD format (current year)
        if let Ok(date) =
            NaiveDate::parse_from_str(&format!("{}-{}", local_now.year(), input), "%Y-%m-%d")
        {
            let dt = Local
                .from_local_datetime(&date.and_hms_opt(default_hour, 0, 0)?)
                .single()?;
            return Some(dt.into());
        }

        None
    }

    fn parse_weekday(&self, input: &str) -> Option<Weekday> {
        match input {
            "monday" | "mon" => Some(Weekday::Mon),
            "tuesday" | "tue" => Some(Weekday::Tue),
            "wednesday" | "wed" => Some(Weekday::Wed),
            "thursday" | "thu" => Some(Weekday::Thu),
            "friday" | "fri" => Some(Weekday::Fri),
            "saturday" | "sat" => Some(Weekday::Sat),
            "sunday" | "sun" => Some(Weekday::Sun),
            _ => None,
        }
    }

    fn format_date_for_editing(&self, time: Option<SystemTime>) -> String {
        match time {
            Some(t) => {
                let now = SystemTime::now();
                if let Ok(duration) = t.duration_since(now) {
                    let days = duration.as_secs() / (24 * 60 * 60);
                    let hours = (duration.as_secs() % (24 * 60 * 60)) / (60 * 60);
                    if days > 0 {
                        format!("{days}d")
                    } else if hours > 0 {
                        format!("{hours}h")
                    } else {
                        "1h".to_string()
                    }
                } else if let Ok(duration) = now.duration_since(t) {
                    // Past date - show negative
                    let days = duration.as_secs() / (24 * 60 * 60);
                    let hours = (duration.as_secs() % (24 * 60 * 60)) / (60 * 60);
                    if days > 0 {
                        format!("-{days}d")
                    } else if hours > 0 {
                        format!("-{hours}h")
                    } else {
                        "now".to_string()
                    }
                } else {
                    "0d".to_string()
                }
            }
            None => "".to_string(),
        }
    }

    fn create_task(&self, title: &str) -> Task {
        Task {
            id: Uuid::new_v4().to_string(),
            title: title.to_string(),
            description: String::new(),
            completed: false,
            created_at: std::time::SystemTime::now(),
            due: None,
            defer_until: None,
        }
    }

    fn apply_to_selection<F>(&mut self, mut func: F)
    where
        F: FnMut(&mut Task),
    {
        let indices: Vec<usize> = self.selection().collect();
        for index in indices {
            if let Some(task) = self.state.tasks.get_mut(index) {
                func(task);
            }
        }
        self.sync_to_storage().unwrap_or_else(|e| {
            self.set_error_message(format!("Error syncing tasks: {e}"));
        });
        self.clear_task_selection();
    }

    fn sync_to_storage(&mut self) -> Result<(), DbError> {
        self.state.store.clear()?;
        for task in &self.state.tasks {
            self.state.store.save_task(task)?;
        }
        Ok(())
    }

    // Task list selection methods
    pub fn cursor_next_task(&mut self) {
        self.task_list_state.select_next();
    }

    pub fn cursor_previous_task(&mut self) {
        self.task_list_state.select_previous();
    }

    pub fn cursor_first_task(&mut self) {
        self.task_list_state.select_first();
    }

    pub fn cursor_last_task(&mut self) {
        self.task_list_state.select_last();
    }

    pub fn cursor_task_index(&self) -> Option<usize> {
        self.task_list_state.selected()
    }

    pub fn clear_task_selection(&mut self) {
        self.task_selection.clear();
    }

    pub fn selection(&self) -> SelectionIterator<'_> {
        if !self.task_selection.is_empty() {
            SelectionIterator::Multiple(self.task_selection.iter())
        } else if let Some(selected) = self.task_list_state.selected() {
            SelectionIterator::Single(std::iter::once(selected))
        } else {
            SelectionIterator::Empty
        }
    }

    pub fn toggle_task_selection(&mut self) {
        if let Some(selected) = self.task_list_state.selected() {
            if self.task_selection.contains(&selected) {
                self.task_selection.remove(&selected);
            } else {
                self.task_selection.insert(selected);
            }
        }
    }

    pub fn task_list_state(&mut self) -> &mut TableState {
        &mut self.task_list_state
    }

    pub fn get_task_selection(&self) -> &HashSet<usize> {
        &self.task_selection
    }

    pub fn create_task_below_cursor(&mut self) {
        let new_task = self.create_task("");
        let cursor_index = self.task_list_state.selected().unwrap_or(0);
        let insert_index = if self.state.tasks.is_empty() {
            0
        } else {
            (cursor_index + 1).min(self.state.tasks.len())
        };
        self.state.tasks.insert(insert_index, new_task.clone());
        self.state.editing_task = Some(new_task);
        self.state.editing_field = 0;
        self.task_list_state.select(Some(insert_index));
    }

    pub fn create_task_above_cursor(&mut self) {
        let new_task = self.create_task("");
        let cursor_index = self.task_list_state.selected().unwrap_or(0);
        self.state.tasks.insert(cursor_index, new_task.clone());
        self.state.editing_task = Some(new_task);
        self.state.editing_field = 0;
        self.task_list_state.select(Some(cursor_index));
    }

    pub fn save_editing_task(&mut self) -> Result<(), DbError> {
        if let Some(editing_task) = &self.state.editing_task {
            if let Some(selected_index) = self.task_list_state.selected() {
                if selected_index < self.state.tasks.len() {
                    self.state.tasks[selected_index] = editing_task.clone();
                    self.sync_to_storage()?;
                }
            }
        }
        self.state.editing_task = None;
        Ok(())
    }

    pub fn update_editing_task_field(&mut self, field_index: usize, value: String) {
        // Parse dates outside the mutable borrow to avoid borrowing conflicts
        let parsed_date = if field_index == 2 || field_index == 3 {
            // field_index 2 is due date (5pm), field_index 3 is defer date (8am)
            let is_due_date = field_index == 2;
            self.parse_date_input(&value, is_due_date)
        } else {
            None
        };

        if let Some(ref mut editing_task) = self.state.editing_task {
            match field_index {
                0 => editing_task.title = value,
                1 => editing_task.description = value,
                2 => editing_task.due = parsed_date,
                3 => editing_task.defer_until = parsed_date,
                _ => {}
            }
        }
    }

    pub fn get_editing_task_field(&self, field_index: usize) -> String {
        if let Some(ref editing_task) = self.state.editing_task {
            match field_index {
                0 => editing_task.title.clone(),
                1 => editing_task.description.clone(),
                2 => self.format_date_for_editing(editing_task.due),
                3 => self.format_date_for_editing(editing_task.defer_until),
                _ => String::new(),
            }
        } else {
            String::new()
        }
    }

    pub fn start_editing_current_task(&mut self) {
        if let Some(selected_index) = self.task_list_state.selected() {
            if selected_index < self.state.tasks.len() {
                self.state.editing_task = Some(self.state.tasks[selected_index].clone());
                self.state.editing_field = 0;

                // Load the current field content into input buffer
                let field_content = self.get_editing_task_field(self.state.editing_field);
                self.state.input_buffer = field_content;
            }
        }
    }
}

pub enum SelectionIterator<'a> {
    Multiple(std::collections::hash_set::Iter<'a, usize>),
    Single(std::iter::Once<usize>),
    Empty,
}

impl<'a> Iterator for SelectionIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            SelectionIterator::Multiple(iter) => iter.next().copied(),
            SelectionIterator::Single(iter) => iter.next(),
            SelectionIterator::Empty => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Timelike;
    use std::time::Duration;

    #[test]
    fn test_parse_date_input_relative_days() {
        let app = App::new(crate::types::AppState::default());
        let now = SystemTime::now();

        if let Some(parsed) = app.parse_date_input("2d", true) {
            let expected = now + Duration::from_secs(2 * 24 * 60 * 60);
            // Allow for small timing differences
            assert!(
                parsed.duration_since(expected).unwrap_or(Duration::ZERO) < Duration::from_secs(1)
            );
        } else {
            panic!("Failed to parse '2d'");
        }
    }

    #[test]
    fn test_parse_date_input_relative_hours() {
        let app = App::new(crate::types::AppState::default());
        let now = SystemTime::now();

        if let Some(parsed) = app.parse_date_input("3h", true) {
            let expected = now + Duration::from_secs(3 * 60 * 60);
            // Allow for small timing differences
            assert!(
                parsed.duration_since(expected).unwrap_or(Duration::ZERO) < Duration::from_secs(1)
            );
        } else {
            panic!("Failed to parse '3h'");
        }
    }

    #[test]
    fn test_parse_date_input_empty() {
        let app = App::new(crate::types::AppState::default());
        assert_eq!(app.parse_date_input("", true), None);
        assert_eq!(app.parse_date_input("-", true), None);
        assert_eq!(app.parse_date_input("   ", true), None);
    }

    #[test]
    fn test_parse_date_input_invalid() {
        let app = App::new(crate::types::AppState::default());
        assert_eq!(app.parse_date_input("invalid", true), None);
        assert_eq!(app.parse_date_input("2x", true), None);
        assert_eq!(app.parse_date_input("abc", true), None);
    }

    #[test]
    fn test_update_editing_task_field_dates() {
        let mut app = App::new(crate::types::AppState::default());
        let task = Task {
            id: "test".to_string(),
            title: "Test Task".to_string(),
            description: "Test Description".to_string(),
            completed: false,
            created_at: SystemTime::now(),
            due: None,
            defer_until: None,
        };

        app.state.editing_task = Some(task.clone());

        // Test setting due date
        app.update_editing_task_field(2, "1d".to_string());
        assert!(app.state.editing_task.as_ref().unwrap().due.is_some());

        // Test setting defer_until date
        app.update_editing_task_field(3, "2h".to_string());
        assert!(app
            .state
            .editing_task
            .as_ref()
            .unwrap()
            .defer_until
            .is_some());

        // Test clearing dates
        app.update_editing_task_field(2, "".to_string());
        assert!(app.state.editing_task.as_ref().unwrap().due.is_none());
    }

    #[test]
    fn test_parse_date_input_keywords() {
        let app = App::new(crate::types::AppState::default());

        assert!(app.parse_date_input("today", true).is_some());
        assert!(app.parse_date_input("tomorrow", true).is_some());
        assert!(app.parse_date_input("yesterday", true).is_some());
    }

    #[test]
    fn test_parse_date_input_weekdays() {
        let app = App::new(crate::types::AppState::default());

        assert!(app.parse_date_input("monday", true).is_some());
        assert!(app.parse_date_input("friday", true).is_some());
        assert!(app.parse_date_input("next monday", true).is_some());
        assert!(app.parse_date_input("next friday", true).is_some());
    }

    #[test]
    fn test_parse_date_input_absolute_dates() {
        let app = App::new(crate::types::AppState::default());

        assert!(app.parse_date_input("2024-12-25", true).is_some());
        assert!(app.parse_date_input("12-25", true).is_some());
    }

    #[test]
    fn test_parse_weekday() {
        let app = App::new(crate::types::AppState::default());

        assert_eq!(app.parse_weekday("monday"), Some(chrono::Weekday::Mon));
        assert_eq!(app.parse_weekday("tue"), Some(chrono::Weekday::Tue));
        assert_eq!(app.parse_weekday("friday"), Some(chrono::Weekday::Fri));
        assert_eq!(app.parse_weekday("invalid"), None);
    }

    #[test]
    fn test_parse_date_input_due_vs_defer_times() {
        let app = App::new(crate::types::AppState::default());

        // Test due date (should default to 5pm/17:00)
        if let Some(due_date) = app.parse_date_input("tomorrow", true) {
            let dt = DateTime::<Local>::from(due_date);
            assert_eq!(dt.hour(), 17);
        } else {
            panic!("Failed to parse due date");
        }

        // Test defer date (should default to 8am/08:00)
        if let Some(defer_date) = app.parse_date_input("tomorrow", false) {
            let dt = DateTime::<Local>::from(defer_date);
            assert_eq!(dt.hour(), 8);
        } else {
            panic!("Failed to parse defer date");
        }
    }
}
