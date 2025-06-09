use std::fmt::{self, Display, Formatter};

use native_db::{ToKey, native_db};
use native_model::{Model, native_model};
use serde::{Deserialize, Serialize};

pub type Task = v1::Task;
pub type Status = v1::Status;

pub mod v1 {
    use super::*;

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub enum Status {
        NotStarted,
        InProgress,
        Completed,
        Dropped,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[native_model(id = 1, version = 1)]
    #[native_db]
    pub struct Task {
        #[primary_key]
        pub id: String,
        #[secondary_key]
        pub name: String,
        pub status: Status,
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Status::NotStarted => write!(f, "Not Started"),
            Status::InProgress => write!(f, "In Progress"),
            Status::Completed => write!(f, "Completed"),
            Status::Dropped => write!(f, "Dropped"),
        }
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Task(id: {}, name: {}, status: {})",
            self.id, self.name, self.status
        )
    }
}
