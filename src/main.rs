extern crate rand;
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
	let rndnum=rand::thread_rng().gen_range(1,101);
	println!("secret num:{}",rndnum);
	loop {	
	let mut val =String::new();
	println!("put a value");
	io::stdin().read_line(&mut val).expect("fail to read line");	
    println!("added value {}",val);
	let val: u32 = val.trim().parse() .expect("Please type a number!");

	match val.cmp(&rndnum){
	Ordering::Less => println!("Too small!"),
	Ordering::Greater => println!("Too big!"), 
	Ordering::Equal => {
println!("You win!");
	break;
	}
	}
	
}
}
