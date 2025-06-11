use std::fmt::{self, Display, Formatter};

use native_db::{ToKey, native_db};
use native_model::{Model, native_model};
use serde::{Deserialize, Serialize};

pub type Task = v1::Task;
pub type Status = v1::Status;

pub mod v1 {

    use super::*;

    #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
    pub enum Status {
        Pending,
        InProgress(u64),
        Completed,
        Deferred(u64),
        Dropped,
        OnHold,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[native_model(id = 1, version = 1)]
    #[native_db]
    pub struct Task {
        #[primary_key]
        pub id: String,
        pub name: String,
        pub status: Status,
        pub created_at: u64,
        pub time_spent: u64,
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Task(id: {}, name: {}, status: {}, created_at: {}, time_spent: {})",
            self.id, self.name, self.status, self.created_at, self.time_spent
        )
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Status::Pending => write!(f, "Pending"),
            Status::InProgress(since) => write!(f, "In Progress since {since}"),
            Status::Completed => write!(f, "Completed"),
            Status::Dropped => write!(f, "Dropped"),
            Status::Deferred(until) => write!(f, "Deferred until {until}"),
            Status::OnHold => write!(f, "On Hold"),
        }
    }
}
