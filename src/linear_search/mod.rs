use rand::prelude::*;
use std::io;

pub fn testing() -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let mut nums: Vec<usize> = (1..1000).collect();
    nums.shuffle(&mut rng);
    return nums;
}

struct LSearchData_lists {
    prob_A: Vec<usize>,
    prob_B: Vec<usize>,
    prob_C: Vec<usize>,
    //... add more if you want
}

fn new_LSearchData_lists() -> LSearchData_lists {
    return LSearchData_lists {
        prob_A: testing(),
        prob_B: testing(),
        prob_C: testing(),
    };
}

pub fn lsearchData_lists_cstr(us: &str) {
    /* make new lists */
    let mut prob_list = new_LSearchData_lists();
    /* show Options on terminal */
    println!("Input 'A' or 'B' or 'C'");

    // managing user input
    let mut user_input_string = String::new();
    match io::stdin().read_line(&mut user_input_string) {
        Ok(_goes_into_input_above) => {
            println!("user input : {}", user_input_string);
        }
        Err(_no_updates_is_fine) => {
            println!("Error has occrured");
        }
    }

    /*
    3 choices
    A : linear search
    B : linear search with setinel
    C : nothing for now. Add something if you like
    */
    match user_input_string.trim() {
        "A" => {
            // find 123;
            for i in prob_list.prob_B.iter() {
                if (prob_list.prob_A[*i - 1] == 123) {
                    println!(
                        "current i = {} // number in Vector = {}",
                        i,
                        prob_list.prob_A[*i - 1]
                    );
                    break;
                }
            }
        }
        "B" => {
            // using sentinel method
            prob_list.prob_B.push(12300);
            println!("{}", prob_list.prob_B.len());
            // find 12300 -> which is not in random number range;
            for i in prob_list.prob_B.iter() {
                if (*i == prob_list.prob_B.len() - 1) {
                    println!(
                        "]]]]current i = {} // number in Vector = {}",
                        *i,
                        prob_list.prob_A[*i - 1]
                    );
                    break;
                }
            }
        }
        "C" => {
            println!("{:?}", prob_list.prob_C);
        }
        _ => println!("{}", "no option in match"),
    }
}
