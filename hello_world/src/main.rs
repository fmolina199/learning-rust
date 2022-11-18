use regex::Regex;

fn main() {
    println!("Hello, world!");

	let reg = Regex::new(r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})").unwrap();
	let caps = reg.captures("2011-12-12");

	match caps {
		Some(x) => println!("{}", &x["year"]),
		None    => println!("Not found"),
	}
}
