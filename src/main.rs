use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	for arg in args {
		println!("{}", arg);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn some_unittest() {}
}
