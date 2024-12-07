use core::num;
use std::{collections::btree_map::Range, io};

fn main() {
    let exp_per_lv: Vec<i32> = loading_exp();
    let mut current_lv = String::new();
    loop {
        current_lv.clear();
        println!("Please input your current hunter level:");
        io::stdin()
            .read_line(&mut current_lv)
            .expect("Invalid input!");
        let current_lv_num: usize = match current_lv.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error in input!");
                continue;
            }
        };
        if current_lv_num > 0 && current_lv_num < 51 {
            let mut sum = 0;
            for i in (current_lv_num..50) {
                sum += &exp_per_lv[i];
            }
            println!("EXP to max out: {sum}");
            continue;
        } else {
            println!("Invalid input!");
            continue;
        }
    }
}

fn loading_exp() -> Vec<i32> {
    let mut data: Vec<i32> = Vec::new();
    loop {
        if data.len() == 50 {
            break;
        }
        if data.len() < 1 {
            // lv 1
            data.push(0);
            continue;
        }
        if data.len() < 3 {
            // lv 2-3
            data.push(100);
            continue;
        }
        if data.len() < 7 {
            // lv 4-7
            data.push(200);
            continue;
        }
        if data.len() < 18 {
            // lv 8-18
            data.push(300);
            continue;
        }
        if data.len() < 25 {
            // lv 19-25
            data.push(400);
            continue;
        }
        if data.len() < 35 {
            // lv 26-35
            data.push(500);
            continue;
        }
        if data.len() < 50 {
            // lv 36-50
            data.push(600);
            continue;
        }
    }
    data
}
