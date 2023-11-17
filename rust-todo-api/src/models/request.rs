use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Request {
    pub todo: String,
    pub completed: bool,
}
