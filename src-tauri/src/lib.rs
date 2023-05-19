use diesel::sqlite::SqliteConnection;
use diesel::{prelude::*, sqlite::Sqlite};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use std::env;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut connection = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Failed to use database {}", database_url));

    run_migrations(&mut connection);

    return connection;
}

fn run_migrations(connection: &mut impl MigrationHarness<Sqlite>) {
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Failed to run db migrations");
}

pub mod models;
pub mod schema;
