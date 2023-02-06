use rocket::catch;
use rocket::{Request, serde::json::Json};

use serde::{Serialize};

#[derive(Serialize)]
struct NotFound {
	message: String,
	debug: String,
}


#[catch(404)]
pub async fn not_found(req: &Request<'_>) -> Json<NotFound> {
	Json(NotFound {
		message: format!("Not found!"),
		debug: format!("{}", req.uri()),
	})
}
