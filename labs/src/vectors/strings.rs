pub fn string_len() {
    let s  = String::from("hello, 杨瑞");

    println!("{}", s.len());


    println!("{:?}", s.as_bytes());

    for i in s.chars() {
        print!("{} ", i);
    }
}


#[cfg(test)]
mod tests {
    use crate::vectors::strings;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);

        strings::string_len();
    }
}