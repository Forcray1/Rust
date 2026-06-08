fn first_word(s: &str) -> &str{
	for (i, c) in s.char_indices() {
		if c == ' ' {
			return &s[..i];
		}
	}
	s	
}

fn main(){
	let text: &str = "hello world";
	let res = first_word(text);
	println!("{}", res);
}