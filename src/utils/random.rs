extern crate rand;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

// Random integer
pub fn random_integer(max_int: i32) -> i32 {
	let mut rng = thread_rng();
	let n: u32 = rng.gen_range(1..max_int as u32);

	n as i32
}

// Random string generates a random string of length n
// Alphanumeric = [A-Z], [a-z], [0-9]
pub fn random_string(n: usize) -> String {
	let mut rng = thread_rng();
	let rand_string: String = (0..n).map(|_| rng.sample(Alphanumeric) as char).collect();

	rand_string
}
