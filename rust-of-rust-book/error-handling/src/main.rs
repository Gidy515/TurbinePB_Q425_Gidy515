use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};
fn main() {
    // Unrecoverable errors with panic
   // panic!("Crash and burn"); // calling panic in a simple program

    //let v = vec![1, 2, 3, 4];
    //v[99];  // Attepting to access an element beyond the end of a vector, which will cause a call to panic!

    /*// Recoverable Errors with Result
    enum result<T, E> {
        Ok(T), // T reps the type of the value that will be returned in a success
        Err(E) // The error messege output in case of a failure
    }*/

    /*let greeting_file_result = File::open("hello.txt");

    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error);
        }
    };*/
    //  Handling different kinds of errors in different ways
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
        match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Problem creating the file: {:?}", e),
        }
    }
        other_error => {
            panic!("Problem opening the file: {:?}", other_error);
        }
    },
};

let greeting_file = File::open("hello.txt").unwrap(); // If we run this code without a hello.txt file, we’ll see an error message from the panic! call that the unwrap method makes
let greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project");
/*We use expect in the same way as unwrap: to return the file handle
or call the panic! macro. The error message used by expect in its
call to panic! will be the parameter that we pass to expect, rather
than the default panic! message that unwrap uses. */
}

// A function that returns errors to the calling code using match
fn read_username_from_file() -> Result<String, io::Error> {
     let username_file_result = File::open("hello.txt");
     let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
     };
     let mut username = String::new();
        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
     }
}