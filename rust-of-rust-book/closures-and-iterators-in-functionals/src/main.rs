// to demonstrate some work with closures, we are doing a shirt giveaway project in which  
use core::num;
use std::{thread, time::Duration};

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Blue => num_blue += 1,
                ShirtColor::Red => num_red += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn add_one_v1 (x: u32) -> u32 { x + 1 }
// let add_one_v2 = |x: u32| -> u32 { x + 1 };
// let add_one_v3 = |x| { x + 1 };
// let add_one_v4 = |x| x + 1 ;

fn main() {
    // println!("We are iterating and closing, no?");

    // closures: anonymous functions that capture their environment
    let store = Inventory {
        shirts: vec![
            ShirtColor::Blue,
            ShirtColor::Red,
            ShirtColor::Blue,
        ],
    }; 

    let user_pref1 = Some(ShirtColor::Red);
    let give_away1 = store.giveaway(user_pref1);

    println!("The choice of user 1 is {:?} and his choice is {:?}", user_pref1, give_away1);
    
    let user_pref2 = None;
    let give_away2 = store.giveaway(user_pref2);

    println!("The choice of user 2 is {:?} and his choice is {:?}", user_pref2, give_away2);

    // optional: annotating type on the closure parameters and the return type
    let expensive_closure = |num: i32| -> i32 {
    println!("calculating slowly..");
    thread::sleep(Duration::from_secs(2));
    num
};

let example_closure = |x| x;
let s = example_closure(String::from("hello"));
// let n = example_closure(5); err because it has already been declared as string, won't change type

let list = vec![1, 2, 3];
println!("Before defining closure: {:?}", list);

// let mut borrows_mutably = || list.push(7);
// borrows_mutably();

let only_borrows = || println!("From closure {:?}", list);
println!("Before calling closure {:?}", list);
only_borrows();
println!("After calling closure: {:?}", list);

// Using move to force the closure for the thread to take ownership of list
thread::spawn(move || {
    println!("From thread {:?}", list)
}).join().unwrap()
}
