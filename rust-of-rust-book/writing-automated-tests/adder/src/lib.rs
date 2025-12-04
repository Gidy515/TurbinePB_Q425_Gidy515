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
}
