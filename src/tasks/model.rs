use crate::db;
use crate::error_handler::CustomError;
use crate::schema::tasks;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "tasks"]
pub struct Task {
    pub name: String,
    pub time: chrono::NaiveDateTime,
    pub severity: i32,
    pub priority: i32,
    pub completed: i32,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Tasks {
    pub id: i32,
    pub name: String,
    pub time: chrono::NaiveDateTime,
    pub severity: i32,
    pub priority: i32,
    pub completed: i32,
}

impl Tasks {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let tasks = tasks::table.load::<Tasks>(&conn)?;
        Ok(tasks)
    }
    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let task = tasks::table.filter(tasks::id.eq(id)).first(&conn)?;
        Ok(task)
    }
    pub fn create(task: Task) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let task = Task::from(task);
        let task = diesel::insert_into(tasks::table)
            .values(task)
            .get_result(&conn)?;
        Ok(task)
    }
    pub fn update(id: i32, task: Task) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let task = diesel::update(tasks::table)
            .filter(tasks::id.eq(id))
            .set(task)
            .get_result(&conn)?;
        Ok(task)
    }
    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(tasks::table.filter(tasks::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}

impl Task {
    fn from(task: Task) -> Task {
        Task {
            name: task.name,
            time: task.time,
            severity: task.severity,
            priority: task.priority,
            completed: task.completed,
        }
    }
}