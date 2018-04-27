
use std::ops::Deref;
pub struct MyBox<T>(T);

impl<T> MyBox<T> {

    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}


impl<T> Drop for MyBox<T> {

	fn drop(& mut self) {
		println!("Drop train is being called for MyBox<T> ");
	}
}

pub struct MyStruct {
	pub i:u32,
}