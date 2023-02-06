
use crate::models::todos::Todos;
use rocket::serde::json::Json;
use rocket::{ get, post, put, delete };

#[get("/<id>")]
pub async fn get_todo(
	id: i32,
) -> Json<Todos> {
	Json(Todos{
		id: id,
		text: "test 1st.".into(),
		done: false,
	})
}

#[post("/")]
pub async fn post_todos() -> String {
	"create todo".into()
}

#[put("/<id>")]
pub async fn put_todos(id: i32) -> String {
	"update todo".into()
}

#[delete("/<id>")]
pub async fn delete_todos(id: i32) -> String {
	"delete todo".into()
}
