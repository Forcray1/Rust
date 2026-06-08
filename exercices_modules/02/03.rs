// **D3. Borrow vs move in functions.** Write three functions over a `String`:
// - `fn inspect(s: &String) -> usize` — returns its length, caller keeps the string.
// - `fn shout(s: &mut String)` — appends `"!"`.
// - `fn consume(s: String)` — takes ownership and prints it (string unusable afterward).
// Call all three from `main` in an order that compiles. Why must `consume` be called last?

fn main(){
	let mut text = String::from("test");
	fn inspect(s: &String) -> usize {
		s.len()
	}
	println!("{}", inspect(&text));
	fn shout(s: &mut String){
		s.push('!');
	}
	shout(&mut text);
	fn consume(s: String){
		println!("{}", s)
	}
	consume(text)
}