fn main() {
    panic!("Crash and burn"); // calling panic in a simple program

    let v = vec![1, 2, 3, 4];
    v[99];  // Attepting to access an element beyond the end of a vector, which will cause a call to panic!
}
