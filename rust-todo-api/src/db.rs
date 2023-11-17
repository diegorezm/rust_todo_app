use crate::models::request::Request;
use crate::models::todos::Todo;
use rusqlite::{Connection, Error, Result};
use uuid::Uuid;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("src/db/todo.db")?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS todos (
                id TEXT PRIMARY KEY UNIQUE,
                todo TEXT NOT NULL,
                completed TEXT NOT NULL
            )",
            [],
        )?;
        Ok(Database { conn })
    }

    pub fn create_todo(&self, req: Request) -> Result<Todo, Error> {
        let uuid = Uuid::new_v4();
        let new_todo = Todo {
            id: uuid.to_string(),
            todo: req.todo,
            completed: req.completed,
        };
        self.conn.execute(
            "INSERT INTO todos (id, todo, completed) VALUES (?1, ?2, ?3)",
            &[
                &new_todo.id,
                &new_todo.todo,
                &(new_todo.completed.to_string()),
            ],
        )?;
        Ok(new_todo)
    }

    pub fn get_all_todos(&self) -> Result<Vec<Todo>, Error> {
        let mut stmt = self.conn.prepare("SELECT id, todo, completed FROM todos")?;
        let todo_iter = stmt.query_map([], |row| {
            let raw_complted: String = row.get(2)?;
            let completed = match raw_complted.as_str() {
                "true" => true,
                "false" => false,
                _ => false,
            };

            Ok(Todo {
                id: row.get(0)?,
                todo: row.get(1)?,
                completed,
            })
        })?;

        let mut all_todos = Vec::new();
        for todo in todo_iter {
            all_todos.push(todo?);
        }

        Ok(all_todos)
    }
    pub fn delete_todo(&self, id: &str) -> Result<(), Error> {
        let mut stmt = self.conn.prepare("SELECT id FROM todos WHERE id = ?")?;
        let mut rows = stmt.query(&[id])?;
        if let Some(_) = rows.next()? {
            self.conn.execute("DELETE FROM todos WHERE id = ?", &[id])?;
            Ok(())
        } else {
            Err(Error::QueryReturnedNoRows)
        }
    }

    pub fn edit_todo(&self, req: &Todo) -> Result<Todo, Error> {
        let update_query = "UPDATE todos SET todo = ?, completed = ? WHERE id = ? ";
        self.conn.execute(
            update_query,
            &[&req.todo, &req.completed.to_string(), &req.id],
        )?;

        let updated_todo =
            self.conn
                .query_row("SELECT * FROM todos WHERE id = ?", [&req.id], |row| {
                    let raw_complted: String = row.get(2)?;
                    let completed = match raw_complted.as_str() {
                        "true" => true,
                        "false" => false,
                        _ => false,
                    };
                    Ok(Todo {
                        id: row.get(0)?,
                        todo: row.get(1)?,
                        completed,
                    })
                });
        match updated_todo {
            Ok(todo) => Ok(todo),
            Err(err) => Err(err),
        }
    }

    pub async fn toggle_completed(&self, id: &str) -> Result<(), Error> {
        let todo = self
            .conn
            .query_row("SELECT * FROM todos WHERE id=?", &[&id], |row| {
                let raw_complted: String = row.get(2)?;
                let completed = match raw_complted.as_str() {
                    "true" => true,
                    "false" => false,
                    _ => false,
                };
                Ok(Todo {
                    id: row.get(0)?,
                    todo: row.get(1)?,
                    completed,
                })
            });
        match todo {
            Ok(todo) => {
                let completed = todo.completed;
                let _ = self.conn.execute(
                    "UPDATE todos SET completed=? WHERE id=?",
                    [
                        &(if completed {
                            "false".to_string()
                        } else {
                            "true".to_string()
                        }),
                        id,
                    ],
                );
                Ok(())
            }
            Err(err) => Err(err),
        }
    }
}
