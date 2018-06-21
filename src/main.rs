extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
mod module1;
use module1::MyBox;

use std::rc::Rc;
use std::cell::RefCell;
mod LinkList;


fn main() {
	LinkList_testing()
	//ref_testing();	
	//match_testing();
	//module_testing();
	//Rfcell_testing();
	//reference_Count_Testing()
	//smarpointer_derefernce_testing()
	//static_lifetime_testing();

	//lifetime_ann_fn_testing()
	//trait_bound_with_vec_testing();
	//trait_bound_testing();
	//trait_testing();
	//generic_testing();
    //hashmap_testing();
    //string_testing()
    //vector_testing();
	//test_classes();
	//string_slice_test()
	//takes_ownership_test();
	//borrow_read_ownership_test();
	//borrow_write_ownership_test();
}

fn LinkList_testing()
{
	//let list : Box<LinkList::LinkedList> = Box::new(LinkList::LinkedList{});
	let mut list =  LinkList::LinkedList{head:None};
	loop
	{
		println!("Press 1: to add the element in linklist \n
				  Press 2: to print the linked list \n
				  Press 3: done");
		
		let mut optchoosen = String::new();
        io::stdin().read_line(&mut optchoosen).expect("reading from stdin failed");
		match optchoosen.as_ref()
		{
			"1" =>{
				let mut name=String::new();
				let mut age=String::new();
				println!("Enter name");
				io::stdin().read_line(&mut name).expect("reading from stdin failed");
				println!("Enter age");				
				io::stdin().read_line(&mut age).expect("reading from stdin failed");
				let info = LinkList::information{name:name,age:age.parse::<i32>().expect("expected age as int")};
				list.add(info);
				

			},
			"2" =>
			{

			},
			"3" => break,
			_ => {
				println!{"Wrong option selection please select again"};
			}
		}
		
	}
}



fn ref_testing()
{
	 let x = 123;
    let &x_ref_1 = &x;
    let ref x_ref_2 = &x;
    //let & x_ref_3 = x; // same error as when matching &x
	let x_ref_3 = &x;
    let ref x_ref_4 = x;
	println!("print value:{}",x);
	println!("print value:{}",x_ref_1);
	println!("print value:{}",*x_ref_2);
	println!("print value:{}",*x_ref_3);
	println!("print value:{}",*x_ref_4);
}

 use item_list::{pair};
enum item_list {
	pair{x:u32,y:Box<item_list>},
Nil
}

fn match_testing() {
	//let list=(cons 1 (cons 2 (cons 3 nil)));
	//let itemlist = pair(1,pair(2,pair(3,pair(4,Nil))));
	//let itemlist = pair(1,Box<item_list>(Nil));
	let itemlist = pair{x:1,y:Box::new(pair{x:2,y:Box::new(pair{x:3,y:Box::new(item_list::Nil)})})};
	findvalue(itemlist,2);
	//findvalue(item_list::Nil,12);
	
}

fn findvalue(list:item_list,val:u32)
{
	
	match list {
		pair{x: v, y:_} if v == val => println!("find the matched value:"),
		pair{x:_,y:x} => { println!("movingto next iteration");findvalue(*x,val)},
				item_list::Nil=>println!("cant able to find the matched value"),
	}
}



fn module_testing() {
	let mystruct = module1::MyStruct{i:2};	
}





use List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn Rfcell_testing() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

	println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c); 
	
    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}


struct refCount<T>
{	
	val : T,
}

fn reference_Count_Testing()
{
	//let rcI =refCount{val:32};
	//println!("count after creating c = {}", rcI.val);
	
	let rcI =refCount{val:String::from("this is a new string")};
	println!("count after creating c = {}", rcI.val);
	
	
	let rc=Rc::new(refCount{val:32});
	println!("count after creating c = {}", Rc::strong_count(&rc));
	
	let b=rc.clone();
	println!("count after creating c = {}", Rc::strong_count(&rc));
	{
	let c=rc.clone();
	println!("count after creating c = {}", Rc::strong_count(&c));
	
	}
	println!("count after creating c = {}", Rc::strong_count(&rc));
	
}

fn smarpointer_derefernce_testing()
{
	
	let x=5;
	let y=&x;	
	
	assert_eq!(5,x);	
	assert_eq!(5,*y);
	
	let y=MyBox::new(x);
	assert_eq!(5,*y);
	
}
fn static_lifetime_testing()
{
	let result;
	{
		let s:&'static str="this has static lifetime";
		result =returnsamevalue(&s);	
		//let s="this has static lifetime";
		//let s="this has static lifetime";
		//result =returnsamevalue(&s);	
		//let string1:&'static String = String::from("long string is long");
		//result =returnsamevalue(string1.as_str());	
	}
	println!("The longest string is {}", result);
	
}

fn returnsamevalue(s:&str) ->&str{
	s
}
/*
//lifetime annotation rule
1. Each parameter that is a reference gets its own lifetime parameter. In other words, 
a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32), a 
function with two arguments gets two separate lifetime parameters: 
fn foo<'a, 'b>(x: &'a i32, y: &'b i32), and so on.

2. If there is exactly one input lifetime parameter, that lifetime is assigned to all 
output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.

3. If there are multiple input lifetime parameters, but one of them is &self or &mut self 
because this is a method, then the lifetime of self is assigned to all output lifetime 
parameters. This makes writing methods much nicer.

*/
/*
*Lifetime Annotations in Function Signatures
*/


fn lifetime_ann_fn_testing()
{
	//This will pass
	let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }	
	
	//this will give compile time error: borrowed value does not live long enough
	/*
	let string1 = String::from("long string is long");
	let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());        
    }
	println!("The longest string is {}", result);
	*/
	

}


//fn longest(x:& str,y:& str) -> & str -->will not work as rust compiler dont know
// about the life time of the variable so it will give compile time error.
fn longest<'a>(x:&'a str,y:&'a str) -> &'a str
{
	if x.len() > y.len() {
        x
    } else {
        y
    }

}


/*
*
*	Second Implementing  Traits Bound
*	We can bound a trait to Generiv Types
*/

fn trait_bound_with_vec_testing() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/*
*
*	Implementing  Traits Bound
*	We can bound a trait to Generiv Types
*/
fn trait_bound_testing() {
	let news_article = NewsArticle{
	headline: String::from("this is News Aritcle Headline"),
	location: String::from("this is News Aritcle location"),
	author: String::from("this is News Aritcle author"),
	content: String::from("this is News Aritcle content"),
	};	
	
	let news_article_nosummarization = NewsArticleNoSummarization{
										headline: String::from("this is News Aritcle Headline"),
										};

	trait_bound_inside(&news_article);
	trait_bound_outside(&news_article);
	
	let news_article_nosummarization = NewsArticleNoSummarization{
										headline: String::from("this is News Aritcle Headline"),
										};
	//this will give an error as no summarization is bound to struct news_article_nosummarization								
	//trait_bound_inside(news_article_nosummarization);

}
pub fn trait_bound_inside<T:Summarizable>(item:&T){
	println!("trait bound inside implementation {}",item.summary());
}

pub fn trait_bound_outside<T>(item:&T)
		where T:Summarizable,
{
println!("trait bound outside implementation {}",item.summary());
}

/*
*
*	Implementing  Traits
*
*/
#[allow(dead_code)]
fn trait_testing(){
	let news_article = NewsArticle{
	headline: String::from("this is News Aritcle Headline"),
	location: String::from("this is News Aritcle location"),
	author: String::from("this is News Aritcle author"),
	content: String::from("this is News Aritcle content"),
	};
	
	println!("NewsAticle author_summary:{}, summary:{}",news_article.author_summary(),news_article.summary());
	
	let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
	};
	
	println!("tweet author_summary:{}, summary:{}",tweet.author_summary(),tweet.summary());
}


pub trait Summarizable {
	fn author_summary(&self)->String;
	fn summary(&self)->String {
		String::from(" Generic summary.....")
	}
}

pub struct NewsArticleNoSummarization
{
    pub headline: String,
 }
 

pub struct NewsArticle
{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
	fn author_summary(&self)->String {
		format!(" NewsArticle author summary is : {}, by {} ({})", self.headline, self.author, self.location)	
	}
	
	fn summary(&self)->String {
		format!("News article summary summary is : {}, by {} ({})", self.headline, self.author, self.location)	
	}
	
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
	fn author_summary(&self)->String {
	 format!(" Tweet author summary {}: {}", self.username, self.content)
	}
}

/*
*
*	Generic Programming
*
*/
struct Point<T, U> {
    x: T,
    y: U,
}
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

#[allow(dead_code)]
fn generic_testing() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}


/*
*
*	Hash Map
*
*/
#[allow(dead_code)]
fn hashmap_testing()
{
    let field_name = String::from("Favorite color");
	let field_value = String::from("Blue");
	let mut map = HashMap::new();
	map.insert(field_name, field_value);
	println!("{:?}", map);
	//this will give an error as we didnt insert in by the reference
	//println!("{}",field_value);

    let mut hash = HashMap::new();
    //inserting a value into hash
    hash.insert(String::from("Blue"), 10);
    //inserting a value if the value doesnt exist
    hash.entry(String::from("Blue")).or_insert(50);
    hash.entry(String::from("Yellow")).or_insert(50);

    println!("{:?}", hash);


    let text = "hello world wonderful world"; 
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0); *count += 1;
    }
    println!("{:?}", map);


    //creating hash map from 2 vector
    let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];
let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);
}

#[allow(dead_code)]
fn string_testing()
{
    //string a new string
    let mut _s =String::new();
    //convert string literal to String
    let _s ="string litral to String".to_string();
    //another method to convert string literal to string
    let _s=String::from("string literal to string");

    //experiment 
    let mut s1 = String::from("foo"); 
    let s2 = "bar";
    s1.push_str(&s2);
    println!("s1 is {},s2 is {}",s1, s2);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    //in case of + ->fn add(self, s: &str) -> String  is called and as you see
    //1. it is calling self thats why  s1 lose its ownership and cant be used.
    //2. in case of s2 ,compiler can coerce the &String argument into a &str .
    //println!("s1 is {},s2 is {} s3 is {}",s1, s2, s3);
    println!("s2 is {} s3 is {}", s2, s3);


    //we can also use format!
    //and doesnt take ownership
    let s1 = String::from("tic");
    let s2 = String::from("tac"); 
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s is {} s1 is {},s2 is {} s3 is {}",s,s1, s2, s3);
}

#[allow(dead_code)]
fn vector_testing()
{
    let mut v :Vec<i32>=Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    for i in &v{
    println!("vector value:{}",i);
    }
    printvector(&v);
    printchangevalue(& mut v,5);

    let mut  _vmacro =vec![100,200,300];
    printvector(&_vmacro);
    printchangevalue(& mut _vmacro,5);

}

#[allow(dead_code)]
fn printvector(v:&Vec<i32>){
    for i in v.iter(){
    println!("vector value:{}",i);
    }
}

#[allow(dead_code)]
fn printchangevalue(v:&mut Vec<i32>,inc:i32)
{
    for i in  v.iter_mut(){
        *i+=inc;
        println!("vector change value:{}",*i);
    }
}
/*
 *
 * classes testing
 * 
 *
 */

#[allow(dead_code)]
fn test_classes()
{
	//rec is printing value because of #[derive(Debug)] define in struct class
	let rec= create_rectangle(10,15);
	println!("Area of the Rectangle:{:?} is {}",rec,rec.area());
	
	//rec_static is printing value because of #[derive(Debug)] define in struct class
	let rec_static = Rectangle::instance(12,13);
	println!("Area of the Rectangle:{:?} is {}",rec_static,rec_static.area());
	
}
//1.This is creating an instance
//2. As this is statement this is returning a Rectange Instance
//3. we dont need to define length=length rust can take care about it.
fn create_rectangle(length:u32,breath:u32)->Rectangle
{
 Rectangle{
 length,
 breath
 } 
}

#[derive(Debug)]
struct Rectangle {
 length:u32,
 breath:u32,
 }
 
 impl Rectangle
 {
	//it is like a static member
	fn instance(length:u32,breath:u32)->Rectangle{
		Rectangle{length,breath}
	}
	fn area(&self) ->u32{
		self.length*self.breath
	}
  
 }
 
 

#[allow(dead_code)]
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

#[allow(dead_code)] 
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
