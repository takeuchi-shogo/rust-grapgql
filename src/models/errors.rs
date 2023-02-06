#[derive(Debug)]
pub enum ApiError {
	NotFound,
	InternalServerError,
}


// impl Error for ApiError {
// 	fn description(&self) -> &str {
// 		match *self {
// 			ApiError::NotFound => "not found",
// 			ApiError::InternalServerError => "internal server error",
// 		}
// 	}
// }
