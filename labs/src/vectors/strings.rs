pub fn string_len() {
    let s  = String::from("hello, 杨瑞");

    println!("{}", s.len());


    println!("{:?}", s.as_bytes());

    for i in s.chars() {
        print!("{} ", i);
    }
}