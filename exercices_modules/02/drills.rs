fn main(){
	// (a)
	let s = String::from("hi");
	let s2 = s;
	println!("{s2}");

	// (b)
	let mut v = vec![1, 2, 3];
	let _first = &v[0];
	v.push(4);
	let first = &v[0];
	println!("{first}");

	// (c)
	let s = String::from("hi");
	let r1 = &s;
	let r2 = &s;
	println!("{r1} {r2}");
}