use std::vec;

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

fn generic_largest<T: PartialOrd>(list: &[T]) -> &T { // The essesnce of PartialOrd is to allow comparison operations
    let mut biggest = &list[0];

    for item in list {
        if item > biggest {
            biggest = item;
        }
    }
    biggest
}

// In Struct definition
struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T, U> {
    a: T,
    b: U,
}

// In enum definition
enum Result <T, E> {
    Ok(T),
    Err(E),
}

// In method definitions
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
} 

//  An impl block that only applies to a struct with a particular concrete type for the generic type parameter T
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// A method that uses types different from it's struct definition
struct Point3<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point3<X1, Y1> {
    fn mixup<X2, Y2> (self, other: Point3<X2, Y2>) -> Point3<X1, Y2> {
        Point3 { x: self.x, y: other.y }
    }
}

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

    let integer = Point {x: 8, y: 90};
    println!("integer.x = {}", integer.x());
    let float = Point {x: 4.55, y: 4.1};
    let combined_int_and_float = Point2 {a: 3.9, b: 5};

    let p1 = Point3 { x: 5, y: 10.4 };
    let p2 = Point3 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    //println!("{}, {}, {}", integer, float, combined_int_and_float);
}




