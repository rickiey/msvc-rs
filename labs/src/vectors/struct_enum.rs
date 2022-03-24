
use std::fmt::{Debug, Display};

#[derive(Debug, Clone)]
pub struct My<T>
    where T: Display {
     pub x: T,
     pub y: T,
}

#[derive(Debug,Copy, Clone)]
pub struct M2(i32);

impl<T: Display> MyTrait  for My<T> {
    fn my_method(&self)  {
        println!("{} {}", self.x, self.y);
    }
}

pub fn use_trait() {
    let my = My { x: 1, y: 2 };
    my.my_method();
}

pub trait MyTrait {
    fn my_method(&self){
        println!("Hello");
    }
}

impl<T: Display> Display for My<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "My {{ x: {}, y: {} }}", self.x, self.y)
    }
}

