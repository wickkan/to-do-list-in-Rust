use crate::todo::Todo;
use serde_json::{self};
use std::fs::File;
use std::io::{BufReader, BufWriter};

pub struct TodoList {
    pub todos: Vec<Todo>,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList {
            todos: Vec::new(),
        }
    }

    pub fn add(&mut self, title: String) {
        let id = self.todos.len() + 1;
        let todo = Todo::new(id, title);
        self.todos.push(todo);
    }

    pub fn mark_done(&mut self, id: usize) {
        if let Some(todo) = self.todos.iter_mut().find(|todo| todo.id == id) {
            todo.mark_done();
        }
    }

    pub fn list(&self) {
        for todo in &self.todos {
            println!("{}", todo);
        }
    }

    pub fn save(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create(filename)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &self.todos)?;
        Ok(())
    }

    pub fn load(&mut self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        self.todos = serde_json::from_reader(reader)?;
        Ok(())
    }

    pub fn delete(&mut self, id: usize) {
        self.todos.retain(|todo| todo.id != id);
    }
}

