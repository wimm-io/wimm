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
│  │     Ui      │  │   Events    │  │   Layout    │              │
│  │ Controller  │  │   Handler   │  │   Manager   │              │
│  └─────────────┘  └─────────────┘  └─────────────┘              │
│                                                                 │
│  ┌─────────────┐  ┌─────────────┐                               │
│  │     App     │  │ Help Panel  │                               │
│  │ Controller  │  │   Widget    │                               │
│  └─────────────┘  └─────────────┘                               │
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
- Initial task loading with fallback error handling

**Key Components:**

- Project directory resolution with fallback to current directory
- Database initialization with proper error handling
- UI startup with graceful error recovery
- Initial task loading with empty state fallback

### 2. UI Layer (`ui/`)

The UI layer implements a centralized rendering approach with integrated task list management.

#### Core Components:

**`Ui` (Main Controller)**

- Orchestrates all UI components and rendering
- Manages the main application loop
- Handles terminal initialization and cleanup
- Integrates table-based task list rendering with in-place editing
- Coordinates between App controller and UI widgets

**`App` (Application Controller)**

- Manages application state and business logic operations
- Handles task operations (add, delete, toggle completion)
- Manages task selection state with multi-select support
- Coordinates storage synchronization
- Manages input buffer and in-place field editing
- Provides cursor navigation for table-based task list
- Supports creating tasks above/below current cursor position
- Manages multi-field editing state with field navigation

**`EventHandler`**

- Processes keyboard input events
- Routes events based on current mode (Normal/Insert)
- Translates key presses into application actions
- Handles both single-key and character input

**`LayoutManager`**

- Calculates screen layout areas with vertical constraints (title, main, status)
- Handles floating panel positioning for help overlay
- Manages responsive layout adjustments
- Removed input area to provide more space for table view
- Provides structured layout areas for consistent rendering

#### UI Widgets:

**`HelpPanel`**

- Renders floating help overlay with keyboard shortcuts
- Provides contextual help content with styling
- Manages floating panel positioning and background clearing
- Uses styled text with colors and formatting

### 3. Business Logic Layer

**`Types` (Core Types)**

- Defines core data structures (`Task`, `AppState`, `Mode`)
- Provides serialization/deserialization capabilities with Serde
- Manages application state transitions and defaults
- Supports generic storage backend through type parameters

**Task Management (Integrated)**

- Task business logic integrated directly into App controller
- Handles task creation, updates, and queries through App methods
- Supports in-place editing with field-specific operations
- Manages task positioning and cursor-relative creation

**`TimeTracker` (Placeholder)**

- Future implementation for time tracking with `TimeEntry` structure
- Will manage timer state and duration calculations
- Includes start/stop timer functionality design

**`InputHandler` (Placeholder)**

- Future implementation for advanced input handling
- Will process complex input scenarios and commands

### 4. Storage Layer (`storage/`)

The storage layer implements a trait-based abstraction for data persistence with comprehensive error handling.

**`Db` Trait**

- Defines storage contract with four core operations:
  - `load_tasks()` - Retrieve all tasks as vector
  - `save_task()` - Persist a single task by ID
  - `delete_task()` - Remove a task by ID with validation
  - `clear()` - Remove all tasks atomically

**`SledStorage` (Production)**

- Implements persistent storage using Sled embedded database
- Provides ACID transactions and crash recovery
- Handles JSON serialization/deserialization with error mapping
- Uses path-based initialization with connection error handling

**`MemoryStorage` (Testing/Development)**

- Implements in-memory storage using HashMap
- Provides fast access for testing scenarios
- No persistence across application restarts
- Suitable for development and testing workflows

**`DbError` (Error Handling)**

- Comprehensive error enum using `thiserror`
- Covers connection, serialization, not found, and operation errors
- Automatic conversion from `serde_json::Error`
- Provides detailed error messages for debugging

## Data Flow

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│   User      │    │    UI       │    │    App      │    │   Storage   │
│   Input     │    │ Controller  │    │ Controller  │    │   Layer     │
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

- All operations return `Result` types for comprehensive error handling
- `load_tasks()` returns empty vector if no tasks exist
- `save_task()` overwrites existing tasks with same ID
- `delete_task()` returns `NotFound` error if task doesn't exist
- `clear()` removes all tasks atomically with transaction safety

### UI Component Contracts

**Event Handler Contract:**

```rust
pub fn handle_event<D: Db>(&self, event: Event, app: &mut App<D>)
```

- Processes single event atomically
- Updates application state through app controller
- No direct widget state management (integrated into Ui controller)

**Main UI Render Contract:**

```rust
pub fn run(&mut self) -> Result<(), UiError>
```

- Manages complete application lifecycle
- Handles terminal initialization and cleanup
- Processes event loop until quit signal
- Provides comprehensive error handling

**Widget Render Contract:**

```rust
pub fn render<D: Db>(&self, f: &mut Frame, area: Rect, app: &App<D>)
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
- Defaults to `MemoryStorage` for testing convenience

**`App<T: Db>`:**

- Manages business logic and task operations
- Handles task selection with multi-select support
- Provides cursor navigation and selection iteration
- Synchronizes state changes with storage backend
- Manages error state and user feedback

**State Transitions:**

- Normal ↔ Insert mode transitions
- Help panel visibility toggle
- Task selection state management with multi-select
- Cursor navigation with boundary handling

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

1. **Storage Layer**: Returns `DbError` for all operations with detailed messages
2. **Business Logic**: Propagates storage errors, manages operation failures
3. **UI Layer**: Converts errors to user-friendly messages via App controller
4. **Application**: Handles fatal errors with graceful shutdown and fallback states

## Configuration and Dependencies

### External Dependencies

```toml
[dependencies]
directories = "6.0"          # Cross-platform directory resolution
ratatui = "0.29"             # Terminal UI framework
serde = { version = "1.0", features = ["derive"] }  # Serialization
serde_json = "1.0"           # JSON serialization
sled = "0.34"                # Embedded database
thiserror = "2.0"            # Error handling
uuid = { version = "1.17.0", features = ["v4"] }    # Unique identifiers
```

### Build Configuration

```toml
[package]
name = "wimm"
version = "0.1.0"
edition = "2024"
```

## Task Selection and Navigation

### Selection System

The application supports both single cursor navigation and multi-task selection:

- **Cursor Navigation**: `j/k` for up/down, `g/G` for first/last
- **Multi-Selection**: `x` to toggle individual task selection
- **Operations**: Work on either cursor position or selected tasks

## Table-Based Editing System

### In-Place Field Editing

The UI implements a table-based task display with in-place editing capabilities:

**Table Structure**

- **Status Column**: Shows completion state `[x]` or `[ ]`
- **Title Column**: Displays and allows editing of task titles (40% width)
- **Description Column**: Displays and allows editing of task descriptions (55% width)

**Editing Workflow**

- **Task Creation**: `o`/`O` keys create new tasks below/above current cursor
- **Field Navigation**: `Tab`/`Shift+Tab` moves between Title and Description fields
- **Visual Feedback**: Active editing field highlighted with yellow background
- **State Management**: Editing task state preserved during field navigation
- **Save/Cancel**: `Enter` saves changes, `Esc` cancels and returns to Normal mode

**Display Logic**

- **Normal Mode**: Shows task data from main task list
- **Edit Mode**: Shows current editing task state with field-specific highlighting
- **Row Selection**: Selected tasks highlighted with blue background
- **Multi-Selection**: Selected tasks highlighted with dark gray background
- **Visual Feedback**: Selected tasks highlighted with different background

### Selection Iterator

```rust
pub enum SelectionIterator<'a> {
    Multiple(std::collections::hash_set::Iter<'a, usize>),
    Single(std::iter::Once<usize>),
    Empty,
}
```

Provides unified interface for operating on single or multiple tasks.

## Future Architecture Considerations

### Planned Enhancements

1. **Task Manager Implementation**
   - Rich task operations (search, filter, sort)
   - Task categorization and tagging
   - Bulk operations and batch processing

2. **Time Tracking System**
   - Active timer management with `TimeEntry`
   - Time entry persistence and history
   - Reporting and analytics dashboard

3. **Advanced Input Handling**
   - Vim-like command mode
   - Multi-key shortcuts and keybinding customization
   - Command history and autocompletion

4. **Enhanced UI Components**
   - Separate TaskList widget implementation
   - Theme system and color customization
   - Modal dialogs and confirmation prompts

### Scalability Considerations

- **Database Performance**: Sled provides excellent performance for single-user scenarios up to millions of records
- **Memory Usage**: In-memory task list scales well to thousands of tasks with minimal overhead
- **UI Responsiveness**: Event-driven architecture ensures smooth interactions even with large datasets
- **Storage Growth**: Future automatic cleanup and archiving for completed tasks

## Testing Strategy

### Unit Testing

- Storage implementations with trait-based mocking and dependency injection
- Business logic testing with controlled storage backends
- UI component testing with state simulation and event mocking

### Integration Testing

- End-to-end user workflow simulation
- Storage persistence verification across application restarts
- Error scenario handling and recovery testing

### Performance Testing

- Large task list rendering and navigation performance
- Database operation benchmarks with varying data sizes
- Memory usage profiling under different workloads

This architecture provides a solid foundation for a task management application while maintaining flexibility for future enhancements and extensions. The current implementation focuses on core functionality with a clean separation of concerns, enabling easy extension and modification.
