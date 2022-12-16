fn main() {
	println!("Hello, world!");
	let x: [(i32, String); 1] = [(1, "alalala".to_string())];

	println!("{}", x[0].1);
}
