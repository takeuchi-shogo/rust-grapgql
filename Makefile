# Application running
run:
	cargo run

# Hot reload running
hot:
	cargo watch -x run

# Application build
build:
	cargo build

# Application check
check:
	cargo check

# Rocket environment development mode
dev:
	ROCKET_ENV=development cargo run

# Rocket environment staging mode
staging:
	ROCKET_ENV=staging cargo run

# Rocket environment production mode
production:
	ROCKET_ENV=production cargo run
