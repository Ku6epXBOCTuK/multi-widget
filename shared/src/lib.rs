#![allow(dead_code)]

use specta::Type;

#[derive(Debug, Type)]
pub struct SubTask {
    id: u32,
    title: String,
    completed: bool,
    description: String,
}

#[derive(Debug, Type)]
pub struct Task {
    id: u32,
    title: String,
    completed: bool,
    description: String,
    subtasks: Vec<SubTask>,
}

#[derive(Debug, Type)]
pub struct Project {
    id: u32,
    title: String,
    completed: bool,
    tasks: Vec<Task>,
}

#[derive(Debug, Type)]
pub enum WsEvent {
    AddTask(Task),
    UpdateTask(Task),
    DeleteTask(u32),
}
