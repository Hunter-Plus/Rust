pub fn add_two(a: usize) -> usize {
    internal_adder(a, 2)
}

fn internal_adder(left: usize, right: usize) -> usize {
    left + right
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {value}.");
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {value}.");
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // unit test
    #[test]
    fn internal() {
        let result = internal_adder(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_two_and_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_three_and_two() {
        let result = add_two(3);
        assert_eq!(result, 5);
    }

    #[test]
    fn one_hundred() {
        let result = add_two(100);
        assert_eq!(result, 102);
    }

    #[test]
    fn it_works_with_error() -> Result<(), String> {
        let result = add(2, 2);

        if result == 6 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn another() {
        panic!("failed testing");
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("DOG");
        assert!(
            result.contains("Squirrel"),
            "Greeting did not contain right animal, Squirrel is not there, is was '{result}'"
        );
    }

    #[test]
    fn larger_can_holder_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 2,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(101);
    }
    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    //  substring of the message that the Guess::new function panics with
    fn much_greater_than_100() {
        Guess::new(200);
    }
}
