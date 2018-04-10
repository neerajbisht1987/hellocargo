extern crate rand;
use std::io;
use rand::Rng;
use std::cmp::Ordering;




fn main() {

	string_slice_test()
	//takes_ownership_test();
	//borrow_read_ownership_test();
	//borrow_write_ownership_test();
	
	
}



fn string_slice_test()
{
	let  s=String::from("Hello world");
	let hello =&s[..5];
	let world = &s[6..];
	println!("orig len:{0},hello len:{1}, world len:{2}",s.len(),hello.len(),world.len());
	//s.clear(); error: cannot borrow mutably
	
	let sl="hello world";
	let hl =&sl[..5];
	let wl = &sl[6..];
	println!("orig len:{0},hello len:{1}, world len:{2}",sl.len(),hl.len(),wl.len());
	
	println!("firstword test with string:{}",first_word_slice(&s));
	println!("firstword test with string:{}",first_word_slice(&s[..]));
	println!("firstword test with string literal:{}",first_word_slice(&sl));
	println!("firstword test with string literal:{}",first_word_slice(&sl[..]));
}

#[allow(dead_code)]
fn borrow_write_ownership_test()
{
	let  mut borrow_write_example = String::from("-->borrowing write example");
	borrow_write_ownership(& mut borrow_write_example);
	println!("After Funtion Call: {}",borrow_write_example);

}

#[allow(dead_code)]
fn borrow_read_ownership_test()
{
	let borrow_read_example = String::from("-->borrowing read example");
	borrow_read_ownership(&borrow_read_example);	
	println!("After Funtion Call: {}",borrow_read_example);
}

#[allow(dead_code)]
fn takes_ownership_test()
{
	let take_ownership_example=String::from("-->ownership example");
	takes_ownership(take_ownership_example);	
	//comiple time error "value used here after move"
	//println!("{}",take_ownership_example);
}
 
 //This is acceping String literal as &str, we can define to use string by using &string.
 fn first_word_slice(s:&str) ->&str
 {
	&s[..] 
 }

#[allow(dead_code)]
fn takes_ownership(take_ownership_string :String)
{
	println!("Owner transfer string :{}",take_ownership_string);
}

#[allow(dead_code)]
fn borrow_read_ownership(borrowed_read_string:&String)
{
	println!("Borrowed read String :{}",borrowed_read_string);
	//error:cannot borrow as mutable
	//borrowed_read_string.push_str(" new addition");	
}

#[allow(dead_code)]
fn borrow_write_ownership(borrow_write_string:&mut String)
{
	println!("Borrowed write  String :{}",borrow_write_string);	
	borrow_write_string.push_str(" new addition");
	println!("Borrowed write String with Addition :{}",borrow_write_string);	
		
}


#[allow(dead_code)]
fn guess_game(){
	
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
