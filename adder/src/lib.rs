pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn greeting(name: &str) -> String {
    format!("Hello! {}", name)
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
            panic!(
                "Guess value must be less than or equal to 100, got {}",
                value
            );
        } else if value > 100 {
            panic!("Guess value must be greater than 0, got {}", value);
        }
        Guess { value }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two_sum() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_adds_two_sum_result() -> Result<(), String> {
        let result = add(2, 2);
        if result == 4 {
            Ok(())
        } else {
            Err(String::from("2+2 does not equal 4 error"))
        }
    }

    #[test]
    #[ignore]
    fn fail_exploration() {
        panic!("this test will fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Sahil");
        assert!(
            result.contains("Sahil"),
            "Greeting did not contain name, values was `{}`",
            result
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be greater than 0, got 200")]
    fn guess_greater_than_100() {
        Guess::new(200);
    }
}
