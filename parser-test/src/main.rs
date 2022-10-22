fn main() {
	let path = std::env::args().nth(1).unwrap();
	let mut f = std::fs::File::open(path).unwrap();

	let class = class_parser::parse_class(&mut f);

	println!("{:#?}", class);
}
