use core::num;
use std::vec;


fn main() {    // finding the largest number in a list of numbers
    let number_list = vec![34, 50, 100, 25, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest of them is: {largest}");

    let second_list = vec![90, 60, 105, 125, 64];

    let largest_of_second_list = largest_temp(&second_list);
    println!("largest of the other list is: {largest_of_second_list}");

    let another_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest_temp(&another_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest character is {result}");

    // using generics to find the largest element in a list
    let numberlist2 = vec![50, 60, 200, 800, 450];
    let num_result = generic_largest(&numberlist2);
    println!("The largest number in numberlist2 is {num_result}");

    // using generics to find the largest character in a list
    let charlist2 = vec!['a', 'b', 'y', 'z', 'x'];
    let char_result = generic_largest(&charlist2);
    println!("The largest character in charlist2 is {char_result}");
}

// function that finds the largest number in any list of numbers to avoid duplication
fn largest_temp(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }
    return largest;
}

fn largest_char(char_list: &[char]) -> &char {
    let mut largest_charac = &char_list[0];

    for char_item in char_list {
        if char_item > largest_charac {
            largest_charac = char_item
        }
    }
    return largest_charac;
}

fn generic_largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut biggest = &list[0];

    for item in list {
        if item > biggest {
            biggest = item;
        }
    }
    biggest
}


