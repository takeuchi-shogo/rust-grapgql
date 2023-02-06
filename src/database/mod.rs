use dotenv::dotenv;
use std::env;

use diesel::{mysql::MysqlConnection, Connection, r2d2::{Pool, ConnectionManager}};

#[cfg(mysql)]
#[path = "schemas/mysql/schema.rs"]
pub mod __mysql_schema;

// An alias to the type for a pool of diesel MySQL connection.
pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;

pub fn establish_connection() -> MysqlPool {
	dotenv().ok();

	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	let manager = ConnectionManager::<MysqlConnection>::new(&database_url);
	Pool::new(manager)
		.unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
