use std::time;

use wimm::{
    types::{AppState, Task},
    ui::Ui,
};

fn main() {
    Ui::new(AppState {
        tasks: vec![
            Task {
                id: "1".to_string(),
                title: "Get Milk".to_string(),
                created_at: time::SystemTime::now(),
                completed: false,
                description: "Get some milk from Safeway".to_string(),
            },
            Task {
                id: "2".to_string(),
                title: "Walk Dog".to_string(),
                created_at: time::SystemTime::now(),
                completed: true,
                description: "Take Fido for a walk in the park".to_string(),
            },
            Task {
                id: "3".to_string(),
                title: "Write Code".to_string(),
                created_at: time::SystemTime::now(),
                completed: false,
                description: "Work on the Wimm project".to_string(),
            },
        ],
        ..Default::default()
    })
    .run()
    .unwrap_or_else(|e| eprintln!("Error: {}", e));
}
