use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Users {
	pub id: i32,
	pub name: String,
	pub password: String,
}
