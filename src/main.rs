extern crate rand;
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn takes_ownership(take_ownership_string :String)
{
	println!("Owner transfer string :{}",take_ownership_string);
}


fn borrow_read_ownership(borrowed_read_string:&String)
{
	println!("Borrowed read String :{}",borrowed_read_string);	
	//error:cannot borrow as mutable
	//borrowed_read_string.push_str(" new addition");
	
}

fn borrow_write_ownership(borrow_write_string:&mut String)
{
	println!("Borrowed write  String :{}",borrow_write_string);	
	borrow_write_string.push_str(" new addition");
	println!("Borrowed write String with Addition :{}",borrow_write_string);	
		
}

fn main() {
	let take_ownership_example=String::from("-->ownership example");
	takes_ownership(take_ownership_example);
	
	//comiple time error "value used here after move"
	//println!("{}",take_ownership_example);
	
	
	let borrow_read_example = String::from("borrowing read example");
	borrow_read_ownership(&borrow_read_example);	
	println!("After Funtion Call: {}",borrow_read_example);
	
	let  mut borrow_write_example = String::from("borrowing write example");
	borrow_write_ownership(& mut borrow_write_example);
	println!("After Funtion Call: {}",borrow_write_example);
	
	
	

	let rndnum=rand::thread_rng().gen_range(1,101);
	println!("secret num:{}",rndnum);
	loop {	
	let mut val =String::new();
	println!("put a value");
	io::stdin().read_line(&mut val).expect("fail to read line");	
    println!("added value {}",val);
	let val: u32 = match val.trim().parse() {
			Ok(num) => num,
Err(_) => continue,
			};

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
