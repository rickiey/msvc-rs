

pub fn use_lftm() {

    let mut  a = "asdfasd";
    let mut  b = "asdfasdsgfvasdas";

    print!("{}", longest_str(
        a,
        b
    ));

}

fn longest_str(x:   &str, y: &str) -> String {
    if x.len() > y.len() {
        String::from(x)
    } else {
        String::from(y)
    }
}