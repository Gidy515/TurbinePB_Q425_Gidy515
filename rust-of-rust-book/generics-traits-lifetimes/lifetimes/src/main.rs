fn main() {
    // Preventing Dangling References with Lifetimes
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("r: {r}"); // This line would cause a compile-time error due to dangling reference, it is attempting to use 'r' which references 'x' that is out of scope

    // The Borrow Checker
}
