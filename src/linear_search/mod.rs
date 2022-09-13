use rand::prelude::*;
use std::io;


pub fn testing () -> Vec<u64> {
let mut rng = rand::thread_rng();
let mut nums: Vec<u64> = (1..1000).collect();
nums.shuffle(&mut rng);
// let mut i: usize = 0;
// while i < nums.len() {
//     println!("{}", nums[i]);
//     i= i+1;
// }
// println!("length of nums : {}", nums.len());
return nums;
}

struct LSearchData_lists {
    prob_A : Vec<u64>,
    prob_B : Vec<u64>,
    prob_C : Vec<u64>,
    //... add more if you want
}

fn new_LSearchData_lists () -> LSearchData_lists {
    return LSearchData_lists {
        prob_A : testing(),
        prob_B : testing(),
        prob_C : testing(),
    };
}

pub fn lsearchData_lists_cstr (us : &str) {
    let prob_list = new_LSearchData_lists();
    println!("Input 'A' or 'B' or 'C'");

    let mut user_input_string = String::new();
    match io::stdin().read_line(&mut user_input_string) {
        Ok(_goes_into_input_above) => {},
        Err(_no_updates_is_fine) => {},
    }

    match user_input_string.trim() {
        "A" => {
            println!("{:?}", prob_list.prob_A);
        },
        "B" => {
            println!("{:?}", prob_list.prob_B);
        },
        "C" => {
            println!("{:?}", prob_list.prob_C);
        }
        _ => println!("{}","no option in match")
    }
}