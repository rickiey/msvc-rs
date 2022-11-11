mod lcode;

use lcode::sum_three_number::Solution;

fn main() {


    let mut s = vec![0,-1,1,2,-2,2,4];

    // let solution = sum_three_number::Solution();
    println!("lenght: {:?}", Solution::three_sum(s));

}
