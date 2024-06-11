#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub done: bool,
    pub priority: u8,
}

impl Todo {
    pub fn new(id: usize, title: String, priority: u8) -> Self {
        Todo {
            id,
            title,
            done: false,
            priority,
        }
    }

    pub fn mark_done(&mut self) {
        self.done = true;
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.done { "Done" } else { "Pending" };
        write!(f, "{}: {} [Priority: {}] [{}]", self.id, self.title, self.priority, status)
    }
}

