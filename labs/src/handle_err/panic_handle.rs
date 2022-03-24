use std::fmt::Debug;
use std::fs::File;
use std::io::Read;

pub fn panics<T: Debug>(s : T) -> Result<String, std::io::Error> {
    let v = vec![1, 2, 3];
    println!("{:?}", s);

    // println!("{:?}", v[9]);

    let mut a = File::open("Cargo.toml")?;

    println!("{:#?}", a);
    // println!("{}",a.err().unwrap().to_strig());
    // println!("{:#?}", &a.err().unwrap().kind());
    // println!("{:#?}", a);


    let mut s = String::new();

    a.read_to_string(&mut s)?;

    Ok(s)
}

pub trait MyTrait {
    fn my_trait_method(&self) -> String;
}


impl MyTrait  for Vec<i32>   {
    fn my_trait_method(&self) -> String {
        String::from("MyTrait")
    }
}