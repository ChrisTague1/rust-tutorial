#![allow(dead_code)]

trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    to: u8,
    at: u8,
}

impl Counter {
    fn new(to: u8) -> Self {
        Counter { to, at: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.at < self.to {
            self.at += 1;
            return Some((self.at) as u32);
        } {
            return None
        }
    }
}

trait TypedIter<T> {
    fn typed_next(&mut self) -> Option<T>;
}

impl TypedIter<String> for Counter {
    fn typed_next(&mut self) -> Option<String> {
        if self.at < self.to {
            self.at += 1;
            return Some(self.at.to_string());
        } {
            return None
        }
    }
}

impl TypedIter<u8> for Counter {
    fn typed_next(&mut self) -> Option<u8> {
        if self.at < self.to {
            self.at += 1;
            return Some(self.at);
        } {
            return None
        }
    }
}

pub fn main() {
    let mut counter = Counter::new(10);

    // cannot use an actual for loop since this isn't the real iterator trait
    while let Some(item) = counter.next() {
        println!("Item is {}!", item);
    }

    let mut counter = Counter::new(10);

    // this is very strange because we are manually passing self, which is how functions actually
    // work
    while let Some(item) = <Counter as TypedIter<String>>::typed_next(&mut counter) {
        println!("Item is {}!", item);
    }

    while let Some(item) = <Counter as TypedIter<u8>>::typed_next(&mut counter) {
        println!("Item is {}!", item);
    }
}
