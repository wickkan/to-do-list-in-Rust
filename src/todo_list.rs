// src/todo_list.rs

use crate::todo::Todo;

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
}
