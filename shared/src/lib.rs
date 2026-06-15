#![allow(dead_code)]

use serde::Serialize;
use specta::Type;

#[derive(Debug, Type, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TaskStatus {
    Pending,
    InProgress,
    Done,
}

#[derive(Debug, Type)]
pub struct SubTask {
    id: u32,
    parent: u32,
    title: String,
    status: TaskStatus,
    description: String,
}

#[derive(Debug, Type)]
pub struct Task {
    id: u32,
    parent: u32,
    title: String,
    status: TaskStatus,
    description: String,
    subtasks: Vec<SubTask>,
}

#[derive(Debug, Type)]
pub struct Stage {
    id: u32,
    title: String,
    status: TaskStatus,
    tasks: Vec<Task>,
}

#[derive(Debug, Type)]
pub enum WsEvent {
    AddTask(Task),
    UpdateTask(Task),
    AddSubTask(SubTask),
    UpdateSubTask(SubTask),
    AddStage(Stage),
    UpdateStage(Stage),
    Delete(u32),
}
