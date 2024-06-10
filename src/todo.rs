use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub done: bool,
}

impl Todo {
    pub fn new(id: usize, title: String) -> Self {
        Todo {
            id,
            title,
            done: false,
        }
    }

    pub fn mark_done(&mut self) {
        self.done = true;
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.done { "Done" } else { "Pending" };
        write!(f, "{}: {} [{}]", self.id, self.title, status)
    }
}
