use core::num;

use rand::prelude::*;
pub fn testing () {
let mut rng = rand::thread_rng();
let mut nums: Vec<u64> = (1..100000).collect();
nums.shuffle(&mut rng);
println!("{:?}", nums[0]);
}


pub struct LSearchData_lists {
    prob_A : Vec<i32>,
    prob_B : Vec<i32>,
    prob_c : Vec<i32>,
    //... add more if you want
}

pub fn lsearchData_lists_cstr (us : &str) {
    match us {
        "A" => {
            println!("{}", us)
        },
        "B" => {
            println!("{}", us)
        },
        "C" => {
            println!("{}", us)
        }
        _ => println!("{}","no option in match")
    }
}