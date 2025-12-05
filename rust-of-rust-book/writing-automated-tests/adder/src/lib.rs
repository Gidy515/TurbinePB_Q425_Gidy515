pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Testing Equality with the assert_eq! and assert_ne! Macros
fn adds_two(num: u32) -> u32 {
    let result = num + 2;
    result
}

// Adding custom failure messages
// an example function that greets people by name and we want to test that the name we pass into the function appears in the output:
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// using should_panic to test for panics: The importance of this is to catch cases where your code is supposed to panic under certain conditions, and you want to ensure that it does so correctly.
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        /*if value < 1 || value > 100 {
            panic!("Value must be between 1 and 100, got {}", value);
        }
        Guess { value } // Guess { value: value }, struct_field = function_argument*/
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}", value)
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}", value)
        }
        Guess { value }
    }
}

// Using Result<T, E> in tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7
        };
        let smaller = Rectangle {
            width: 5,
            height: 1
        };
        assert!(larger.can_hold(&smaller)); // 8 > 5 && 7 > 1
    }

    #[test]
    fn larger_cannot_hold_smaller() {
        let larger2 = Rectangle {
            width: 8,
            height: 7
        };
        let smaller2 = Rectangle {
            width: 5,
            height: 1
        };
        assert!(!smaller2.can_hold(&larger2)); // 5 !> 8 && 1 !> 7
    }

    #[test]
    fn testing_adds_two() {
        assert_eq!(5, adds_two(3)); // while assert_eq! checks for equality, assert_ne! checks for inequality between two values for testing purposes
    }

    #[test]
    fn greeting_contains_name() {
        let result = greet("Carol");
        assert!(result.contains("Carol"), // codition to be tested
        "Greeting did not contain name, value was `{result}`", // custom error message if condition is false
    );
    }

    #[test]
    #[should_panic (expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn err_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 does not equal 4"))
        }
    }
}
