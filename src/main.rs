#[macro_use]
extern crate rocket;

use models::users::Users;
use rocket::serde::json::Json;
use rust_graphql::{logs, graphql, database::establish_connection};

#[macro_use]
pub mod database;
pub mod models;
pub mod routes;
pub mod utils;

#[get("/hello")]
async fn hello() -> Json<Vec<Users>> {
	let mut users_json: Vec<Users> = Vec::new();
	for i in 0..10 {
		let u = Users {
			id: i,
			name: "ssss".into(),
			password: utils::password::generater_password(&"password333".into()),
		};
		let json = serde_json::to_string(&u).unwrap();
		let user_json = serde_json::from_str(&json).unwrap();
		users_json.push(user_json)
	}
	Json(users_json)
}

#[get("/users/<id>")]
async fn users(id: i32) -> Json<Vec<Users>> {
	Json(vec![Users {
		id: id,
		name: "rsomfef eomcsa".into(),
		password: utils::password::generater_password(&"password333".into()),
	}])
}

/// Produce an infinite series of `"hello"`s, one per second.
// #[get("/infinite-hellos")]
// fn hello1() -> TextStream![&'static str] {
// 	TextStream! {
// 		let mut interval = interval(Duration::from_secs(1));
// 		loop {
// 			yield "hello";
// 			interval.tick().await;
// 		}
// 	}
// }

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

	logs::logger::setup();
	launch_info();

	launch_rocket().await
}

fn launch_info() {
	println!("/--------------------------------------------------------------------\\");
	println!("|                        Starting Vaultwarden                        |");

	// if let Some(version) = VERSION {
	// 	println!("|{:^68}|", format!("Version {version}"));
	// }

	println!("|--------------------------------------------------------------------|");
	println!("| This is an *unofficial* Bitwarden implementation, DO NOT use the   |");
	println!("| official channels to report bugs/features, regardless of client.   |");
	println!("| Send usage/configuration questions or feature requests to:         |");
	println!("|   https://vaultwarden.discourse.group/                             |");
	println!("| Report suspected bugs/issues in the software itself at:            |");
	println!("|   https://github.com/dani-garcia/vaultwarden/issues/new            |");
	println!("\\--------------------------------------------------------------------/\n");
}

async fn launch_rocket() -> Result<(), rocket::Error> {

	let schema = graphql::schema_build();

	let _rocket = rocket::build()
		.manage(schema)
		.manage(establish_connection())
		.mount(
			"/",
			routes![
				hello, // hello1,
				users,
				graphql::graphiql,
				graphql::graphql_query,
				graphql::graphql_request,
			],
		)
		.mount("/todos", routes![
			routes::todos::get_todo,
			routes::todos::post_todos,
			routes::todos::put_todos,
			routes::todos::delete_todos,
		])
		.register("/", catchers![
			routes::error::not_found
		])
		// .attach()
		.launch()
		.await?;

	Ok(())
}
