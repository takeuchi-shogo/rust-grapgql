table! {
	todos (id) {
		id -> Integer,
		text -> Text,
		done -> Bool,
	}
}

table! {
	users (id) {
		id -> Integer,
		name -> Text,
		password -> Text,
	}
}
