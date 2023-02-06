use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(SimpleObject)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Todos {
	pub id: i32,
	pub text: String,
	pub done: bool,
}
