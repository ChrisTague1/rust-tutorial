use rand;

pub fn add(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(1);
        assert_eq!(result, 2);
    }
}
