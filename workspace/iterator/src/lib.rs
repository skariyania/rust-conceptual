use std::ops::Add;

pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

pub struct Counter {}

// iterator implementation on type
impl Iterator for Counter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }

    // similar way we cound have used generics
    // cons and pro against traits
    // cons: we will need to annotate type in each implementation of generics
    // pro: we can implement generics with different types
    // pub trait Iterator<T> {
    //     fn next(&mut self) -> Option<T>;
    // }
}

// another example using traits
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
