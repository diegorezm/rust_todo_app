use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub id: String,
    pub todo: String,
    pub completed: bool,
}

