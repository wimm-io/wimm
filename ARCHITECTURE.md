# Wimm Task Manager Architecture

## Overview

Wimm is a terminal-based task management application built in Rust using the Ratatui framework. It follows a modular architecture with clear separation of concerns between storage, business logic, and user interface components.

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         Application Layer                       │
│                           (main.rs)                             │
└─────────────────────────┬───────────────────────────────────────┘
                          │
┌─────────────────────────▼───────────────────────────────────────┐
│                      UI Layer (ui/)                             │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐              │
│  │    App      │  │   Events    │  │   Layout    │              │
│  │ Controller  │  │   Handler   │  │   Manager   │              │
│  └─────────────┘  └─────────────┘  └─────────────┘              │
│                                                                 │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐              │
│  │  Task List  │  │ Input Bar   │  │ Help Panel  │              │
│  │   Widget    │  │   Widget    │  │   Widget    │              │
│  └─────────────┘  └─────────────┘  └─────────────┘              │
└─────────────────────────┬───────────────────────────────────────┘
                          │
┌─────────────────────────▼───────────────────────────────────────┐
│                    Business Logic Layer                         │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐              │
│  │    Types    │  │    Tasks    │  │    Time     │              │
│  │   (Core)    │  │   Manager   │  │  Tracking   │              │
│  └─────────────┘  └─────────────┘  └─────────────┘              │
│                                                                 │
│  ┌─────────────┐                                                │
│  │    Input    │                                                │
│  │   Handler   │                                                │
│  └─────────────┘                                                │
└─────────────────────────┬───────────────────────────────────────┘
                          │
┌─────────────────────────▼───────────────────────────────────────┐
│                    Storage Layer (storage/)                     │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐              │
│  │     Db      │  │    Sled     │  │   Memory    │              │
│  │   Trait     │  │  Storage    │  │   Storage   │              │
│  └─────────────┘  └─────────────┘  └─────────────┘              │
└─────────────────────────────────────────────────────────────────┘
```

## Component Details

### 1. Application Layer (`main.rs`)

**Responsibilities:**

- Application bootstrapping and initialization
- Database path resolution using `directories` crate
- Error handling and application lifecycle management

**Key Components:**

- Project directory resolution
- Database initialization
- UI startup and error handling

### 2. UI Layer (`ui/`)

The UI layer implements the Model-View-Controller pattern with widget-based rendering.

#### Core Components:

**`App` (Controller)**

- Manages application state
- Handles business logic operations (add, delete, toggle tasks)
- Coordinates between UI and storage layers
- Manages input buffer and mode transitions

**`EventHandler`**

- Processes keyboard input events
- Routes events based on current mode (Normal/Insert)
- Translates key presses into application actions

**`LayoutManager`**

- Calculates screen layout areas
- Handles floating panel positioning
- Manages responsive layout adjustments

#### UI Widgets:

**`TaskList`**

- Renders task items with completion status
- Manages selection state
- Handles navigation (up/down, first/last)

**`InputBar`**

- Displays input prompt in Insert mode
- Shows error messages in Normal mode
- Handles real-time input display

**`HelpPanel`**

- Renders floating help overlay
- Provides contextual help content
- Manages help visibility state

### 3. Business Logic Layer

**`Types` (Core Types)**

- Defines core data structures (`Task`, `AppState`, `Mode`)
- Provides serialization/deserialization capabilities
- Manages application state transitions

**`TaskManager` (Placeholder)**

- Future implementation for task business logic
- Will handle task creation, updates, and queries
- Planned for future development

**`TimeTracker` (Placeholder)**

- Future implementation for time tracking
- Will manage timer state and duration calculations
- Planned for future development

**`InputHandler` (Placeholder)**

- Future implementation for advanced input handling
- Will process complex input scenarios
- Planned for future development

### 4. Storage Layer (`storage/`)

The storage layer implements a trait-based abstraction for data persistence.

**`Db` Trait**

- Defines storage contract with four operations:
  - `load_tasks()` - Retrieve all tasks
  - `save_task()` - Persist a single task
  - `delete_task()` - Remove a task by ID
  - `clear()` - Remove all tasks

**`SledStorage` (Production)**

- Implements persistent storage using Sled embedded database
- Provides ACID transactions and crash recovery
- Handles serialization/deserialization with JSON

**`MemoryStorage` (Testing/Development)**

- Implements in-memory storage using HashMap
- Provides fast access for testing scenarios
- No persistence across application restarts

## Data Flow

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│   User      │    │    UI       │    │    App      │    │   Storage   │
│   Input     │    │   Events    │    │ Controller  │    │   Layer     │
└──────┬──────┘    └──────┬──────┘    └──────┬──────┘    └──────┬──────┘
       │                  │                  │                  │
       │ Key Press        │                  │                  │
       ├─────────────────►│                  │                  │
       │                  │                  │                  │
       │                  │ Event Handler    │                  │
       │                  ├─────────────────►│                  │
       │                  │                  │                  │
       │                  │                  │ Storage Op       │
       │                  │                  ├─────────────────►│
       │                  │                  │                  │
       │                  │                  │ Result           │
       │                  │                  │◄─────────────────┤
       │                  │                  │                  │
       │                  │ State Update     │                  │
       │                  │◄─────────────────┤                  │
       │                  │                  │                  │
       │ Screen Update    │                  │                  │
       │◄─────────────────┤                  │                  │
```

## Component Contracts

### Storage Contract (`Db` trait)

```rust
pub trait Db {
    fn load_tasks(&self) -> Result<Vec<Task>, DbError>;
    fn save_task(&mut self, task: &Task) -> Result<(), DbError>;
    fn delete_task(&mut self, task_id: &str) -> Result<(), DbError>;
    fn clear(&mut self) -> Result<(), DbError>;
}
```

**Guarantees:**

- All operations return `Result` types for error handling
- `load_tasks()` returns empty vector if no tasks exist
- `save_task()` overwrites existing tasks with same ID
- `delete_task()` returns `NotFound` error if task doesn't exist
- `clear()` removes all tasks atomically

### UI Component Contracts

**Event Handler Contract:**

```rust
pub fn handle_event<D: Db>(&self, event: Event, app: &mut App<D>, task_list: &mut TaskList)
```

- Processes single event atomically
- Updates application state through app controller
- Manages UI widget state (task list selection)

**Widget Render Contract:**

```rust
pub fn render<D: Db>(&mut self, f: &mut Frame, area: Rect, app_state: &AppState<D>)
```

- Renders widget within provided area
- Reads from app state, never mutates directly
- Handles empty/error states gracefully

### Application State Contract

**`AppState<T: Db>`:**

- Generic over storage implementation
- Maintains single source of truth for application state
- Provides immutable access to UI layer
- Manages mode transitions and input buffer

**State Transitions:**

- Normal ↔ Insert mode transitions
- Help panel visibility toggle
- Task selection state management

## Error Handling Strategy

### Error Types

```rust
pub enum DbError {
    ConnectionError(String),
    SerdeError(String),
    NotFound(String),
    OperationFailed(String),
}

pub enum UiError {
    IoError(std::io::Error),
    DbError(storage::DbError),
}
```

### Error Propagation

1. **Storage Layer**: Returns `DbError` for all operations
2. **Business Logic**: Propagates storage errors, adds validation errors
3. **UI Layer**: Converts errors to user-friendly messages
4. **Application**: Handles fatal errors with graceful shutdown

## Configuration and Dependencies

### External Dependencies

- **Ratatui**: Terminal UI framework
- **Sled**: Embedded database
- **Serde**: Serialization framework
- **UUID**: Unique identifier generation
- **Directories**: Cross-platform directory resolution

### Build Configuration

```toml
[package]
name = "wimm"
version = "0.1.0"
edition = "2024"
```

## Future Architecture Considerations

### Planned Enhancements

1. **Task Manager Implementation**
   - Rich task operations (search, filter, sort)
   - Task categorization and tagging
   - Bulk operations

2. **Time Tracking System**
   - Active timer management
   - Time entry persistence
   - Reporting and analytics

3. **Advanced Input Handling**
   - Vim-like command mode
   - Multi-key shortcuts
   - Customizable keybindings

4. **Plugin Architecture**
   - External integrations (GitHub, Jira)
   - Custom storage backends
   - Theme system

### Scalability Considerations

- **Database Performance**: Sled provides excellent performance for single-user scenarios
- **Memory Usage**: In-memory task list scales well to thousands of tasks
- **UI Responsiveness**: Event-driven architecture ensures smooth interactions
- **Storage Growth**: Automatic cleanup and archiving for completed tasks

## Testing Strategy

### Unit Testing

- Storage implementations with trait-based mocking
- Business logic with dependency injection
- UI components with state simulation

### Integration Testing

- End-to-end user workflows
- Storage persistence verification
- Error scenario handling

### Performance Testing

- Large task list rendering
- Database operation benchmarks
- Memory usage profiling

This architecture provides a solid foundation for a task management application while maintaining flexibility for future enhancements and extensions.
