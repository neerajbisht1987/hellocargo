use std::io;


fn main() {
	let mut val =String::new();
	println!("put a value");
	io::stdin().read_line(&mut val).expect("fail to read line");	
    println!("added value {}",val);
}
