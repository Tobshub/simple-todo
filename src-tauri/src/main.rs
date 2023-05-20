// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

use self::models::{NewTodo, Todo};
use diesel::prelude::*;
use simple_todo::{schema::todos, *};

#[tauri::command]
fn api(action: &str, content: Option<&str>, id: Option<i32>) -> Option<Vec<Todo>> {
    let connection = &mut establish_connection();
    match action {
        "get" => {
            let res = get_todos(connection);
            return res;
        }
        "add" => {
            if let Some(content) = content {
                return match add_todo(connection, content) {
                    Ok(_) => None,
                    Err(_) => {
                        println!("Failed to add todo");
                        None
                    }
                };
            }
            return None;
        }
        "delete" => {
            if let Some(id) = id {
                return match delete_todo(connection, id) {
                    Ok(_) => None,
                    Err(_) => {
                        println!("Failed to delete todo");
                        None
                    }
                };
            }
            return None;
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
    let new_todo = NewTodo { content };

    diesel::insert_into(todos::table)
        .values(&new_todo)
        .execute(conn)?;

    println!("Added new to-do");

    return Ok(());
}

fn delete_todo(conn: &mut SqliteConnection, todo_id: i32) -> Result<(), Box<dyn Error>> {
    use self::schema::todos::dsl::*;

    diesel::delete(todos.filter(id.eq(todo_id))).execute(conn)?;

    println!("Deleted to-do with id {}", todo_id);

    return Ok(());
}
