// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

use self::models::{NewTodo, Todo};
use diesel::prelude::*;
use simple_todo::*;

#[tauri::command]
fn api(action: &str, content: Option<&str>) -> Option<Vec<Todo>> {
    let connection = &mut establish_connection();
    match action {
        "get" => {
            let res = get_todos(connection);
            return res;
        }
        "add" => {
            if let Some(content) = content {
                match add_todo(connection, content) {
                    Ok(_) => {
                        return None;
                    }
                    Err(_) => {
                        println!("Failed to add todo");
                        return None;
                    }
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![api])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn get_todos(conn: &mut SqliteConnection) -> Option<Vec<Todo>> {
    use self::schema::todos::dsl::*;

    let found = match todos.load::<Todo>(conn) {
        Ok(data) => data,
        Err(_) => {
            return None;
        }
    };

    return Some(found);
}

fn add_todo(conn: &mut SqliteConnection, content: &str) -> Result<(), Box<dyn Error>> {
    use crate::schema::todos;

    let new_todo = NewTodo { content };

    diesel::insert_into(todos::table)
        .values(&new_todo)
        .execute(conn)?;

    return Ok(());
}
