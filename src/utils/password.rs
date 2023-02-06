extern crate pwhash;

use pwhash::bcrypt;

// Create password
pub fn generater_password(password: &String) -> String {
	bcrypt::hash(password).unwrap()
}

pub fn check_password(password: String, hash_password: String) -> bool {
	// let hash = hashing_password(&password);
	bcrypt::verify(password, &hash_password)
}
