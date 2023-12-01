pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

#[allow(dead_code)]
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello, {name}!")
}

pub fn i_will_panic() {
    panic!("I am panicing");
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

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Chris");

        assert!(
            result.contains("Chris"),
            "I have a custom error, the value was {}",
            result
        );
    }

    #[test]
    #[should_panic(expected = "I am panicing")]
    fn this_test_will_panic() {
        i_will_panic();
    }

    #[test]
    fn another_it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
