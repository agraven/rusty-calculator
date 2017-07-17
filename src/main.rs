mod calculate;

fn main() {
	let result = calculate::parse_string(String::from("-  1*2  ^ 3 /2"), None).unwrap();
    println!("Output was {}", result);
}
