//! WIMM (Where is my mind) - Core library modules
//!
//! This library provides the core functionality for a terminal-based task management application.
//! It's organized into the following modules:
//!
//! - [`types`] - Core data structures for tasks and application state
//! - [`storage`] - Persistent storage abstraction with multiple backends
//! - [`ui`] - Terminal user interface components and rendering
//! - [`input`] - Input handling and event processing
//! - [`time_tracking`] - Time tracking functionality (placeholder for future features)

pub mod input;
pub mod storage;
pub mod time_tracking;
pub mod types;
pub mod ui;
