use std::f32::consts::SQRT_2;

/*fn main() {
    // Preventing Dangling References with Lifetimes
    let r;
    {
        let x = 5;
        r = &x;
    }
    //println!("r: {r}"); // This line would cause a compile-time error due to dangling reference, it is attempting to use 'r' which references 'x' that is out of scope

// Generic lifetimes in functions
let string1 = String::from("abcd");
let string2 = String::from("xyz"); // let string2 = "xyz"; works as well
let result = longest(string1.as_str(), string2.as_str());
println!("The longest string is {result}!");

}

fn longest <'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}*/
fn main () {
    let result;
    let str1 = String::from("longest, no?");
    {
        let str2 = String::from("small string, no?");
        result = longest(str1, str2);
    }
    println!("The longest string is: {result}");

    let result2;
    let str3 = String::from("longest 2, no?");
    {
        let str4 = String::from("small string 2, no?");
        result2 = longest2(str3.as_str(), str4.as_str());
    }
    println!("The longest string is: {result2}");

    let name = String::from("Gideon");
    let user = User {name: &name};
    println!("User's name is: {}", user.name);
}

// A function that returns the longest of two strings

fn longest(a: String, b: String) -> String {
    if a.len() > b.len() {
        return a;
    } else {
        return b;
    }
}

// tweaking the function to use references that will not return well and will need lifetimes
fn longest2<'a>(c: &'a str, d: &'a str) -> &'a str {
    if c.len() > d.len() {
        c
    } else {
        d
    }
}
// This function will not compile because it attempts to return a reference to a value that will go out of scope

// Structs with Lifetime Annotations
struct User <'a>{
    name: &'a str,
}