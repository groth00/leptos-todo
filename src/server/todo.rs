use leptos::{server, ServerFnError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Todo {
    pub id: i32, // serial
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub created: Option<String>,
    pub due_date: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PaginatedTodos {
    pub items: Vec<Todo>,
    pub total: u32,
    pub page: u32,
    pub total_pages: u32,
}

#[cfg(feature = "ssr")]
pub mod ssr {
    pub use chrono::{self, Datelike};
    pub use tokio;
    pub use tokio_postgres;
    pub use tokio_postgres::{Client, NoTls};
}

#[server]
pub async fn get_paginated_todos(page: u32) -> Result<PaginatedTodos, ServerFnError> {
    use self::ssr::*;

    let (client, connection) =
        tokio_postgres::connect("host=localhost dbname=leptos", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let offset = (&page * 10) as i64;
    let stmt = "SELECT id, title, description, to_char(due_date, 'YYYY-MM-DD') FROM todos WHERE completed = false ORDER BY created DESC LIMIT 10 OFFSET $1";

    let todos = client
        .query(stmt, &[&offset])
        .await?
        .into_iter()
        .map(|row| Todo {
            id: row.get(0),
            title: row.get(1),
            description: row.get(2),
            completed: false,
            created: None,
            due_date: row.get(3),
        })
        .collect::<Vec<_>>();

    let total = client
        .query_one("SELECT count(1) FROM todos WHERE completed = false", &[])
        .await?
        .get::<usize, i64>(0) as u32;

    Ok(PaginatedTodos {
        items: todos,
        total,
        page,
        total_pages: (total + 10 - 1) / 10,
    })
}

#[server]
pub async fn add_todo(
    title: String,
    description: String,
    due_date: String,
) -> Result<(), ServerFnError> {
    use self::ssr::*;

    let (client, connection) =
        tokio_postgres::connect("host=localhost dbname=leptos", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let today = chrono::offset::Local::now().date_naive();
    let (year, month, day) = {
        let ymd: Vec<&str> = due_date.split("-").collect();
        (
            ymd[0].parse::<i32>().unwrap_or(today.year()),
            ymd[1].parse::<u32>().unwrap_or(today.month()),
            ymd[2].parse::<u32>().unwrap_or(today.day()),
        )
    };
    let pg_date = chrono::NaiveDate::from_ymd_opt(year, month, day).unwrap_or(today);

    let stmt = "INSERT INTO todos(title, description, due_date) VALUES($1, $2, $3)";
    let _ = client
        .execute(stmt, &[&title, &description, &pg_date])
        .await?;
    Ok(())
}

#[server]
pub async fn complete_todo(id: i32) -> Result<(), ServerFnError> {
    use self::ssr::*;

    let (client, connection) =
        tokio_postgres::connect("host=localhost dbname=leptos", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let stmt = "UPDATE todos SET completed = true WHERE id = $1";
    let _ = client.execute(stmt, &[&id]).await?;
    Ok(())
}

#[server]
pub async fn update_todo(
    id: i32,
    title: String,
    description: String,
    due_date: String,
) -> Result<(), ServerFnError> {
    use self::ssr::*;

    if title == "" {
        return Err(ServerFnError::Args("title cannot be empty".into()));
    }

    if due_date == "" {
        return Err(ServerFnError::Args("due_date cannot be empty".into()));
    }

    let (client, connection) =
        tokio_postgres::connect("host=localhost dbname=leptos", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let today = chrono::offset::Local::now().date_naive();
    let (year, month, day) = {
        let ymd: Vec<&str> = due_date.split("-").collect();
        (
            ymd[0].parse::<i32>().unwrap_or(today.year()),
            ymd[1].parse::<u32>().unwrap_or(today.month()),
            ymd[2].parse::<u32>().unwrap_or(today.day()),
        )
    };
    let pg_date = chrono::NaiveDate::from_ymd_opt(year, month, day).unwrap_or(today);

    let stmt = "UPDATE todos SET title = $1, description = $2, due_date = $3 WHERE id = $4";
    let _ = client
        .execute(stmt, &[&title, &description, &pg_date, &id])
        .await?;
    Ok(())
}

#[server]
pub async fn delete_todo(id: i32) -> Result<(), ServerFnError> {
    use self::ssr::*;

    let (client, connection) =
        tokio_postgres::connect("host=localhost dbname=leptos", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let stmt = "DELETE FROM todos WHERE id = $1";
    let _ = client.execute(stmt, &[&id]).await?;
    Ok(())
}

#[server]
pub async fn search_todo(query: String) -> Result<Vec<Todo>, ServerFnError> {
    use self::ssr::*;

    let (client, connection) =
        tokio_postgres::connect("host=localhost dbname=leptos", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let stmt =
        "SELECT id, title, description, to_char(due_date, 'YYYY-MM-DD') FROM todos WHERE title::tsvector @@ plainto_tsquery($1) AND completed = false";
    let rows = client
        .query(stmt, &[&query])
        .await?
        .into_iter()
        .map(|r| Todo {
            id: r.get(0),
            title: r.get(1),
            description: r.get(2),
            completed: false,
            created: None,
            due_date: r.get(3),
        })
        .collect::<Vec<_>>();
    Ok(rows)
}
