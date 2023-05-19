use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Todo {
    pub id: i32,
    pub content: String,
}

use crate::schema::todos;

#[derive(Insertable)]
#[diesel(table_name = todos)]
pub struct NewTodo<'a> {
    pub content: &'a str,
}
