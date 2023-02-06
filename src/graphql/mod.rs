use async_graphql::*;
use async_graphql::{http::GraphiQLSource, Context, EmptyMutation, EmptySubscription, FieldResult, Schema};
use async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse};
use rocket::{response::content, State};

use crate::models::todos::Todos;

pub struct QueryRoot;

#[Object]
impl QueryRoot {

	// Get Todos List
	async fn todos(&self, ctx: &Context<'_>) -> FieldResult<Vec<Todos>> {
		let todos: Vec<Todos> = vec![Todos {
			id: 1,
			text: "test1".into(),
			done: false,
			},
			Todos {
				id: 2,
				text: "test2".into(),
				done: false,
			}];
		Ok(todos)
	}
	// Get Todo
	async fn todo(&self, ctx: &Context<'_>, id: i32) -> FieldResult<Todos> {
		let todo: Todos = Todos {
			id: id,
			text: "test!!!!!!!!".into(),
			done: true,
		};
		Ok(todo)
	}
}

pub type StarWarsSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[rocket::get("/")]
pub fn graphiql() -> content::RawHtml<String> {
	content::RawHtml(GraphiQLSource::build().endpoint("/graphql").finish())
}

#[rocket::get("/graphql?<query..>")]
pub async fn graphql_query(schema: &State<StarWarsSchema>, query: GraphQLQuery) -> GraphQLResponse {
	query.execute(schema.inner()).await
}

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
pub async fn graphql_request(
	schema: &State<StarWarsSchema>,
	request: GraphQLRequest,
) -> GraphQLResponse {
	println!("=============================================");
	println!("|          GraphQL server running!!         |");
	println!("=============================================");
	request.execute(schema.inner()).await
}

pub fn schema_build() -> Schema<QueryRoot, EmptyMutation, EmptySubscription> {
	// let query = QueryRoot {};
	let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
		.finish();

	schema
}
