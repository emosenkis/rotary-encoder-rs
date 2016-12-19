extern crate futures;

use futures::{Poll, Stream};

#[derive(Copy,Clone,Debug,Eq,PartialEq)]
pub enum Direction {
    CW,
    CCW,
}

// A B
// Clockwise
// 0 0
// 0 1
// 1 1
// 1 0
//
// Counter-clockwise
// 1 0
// 1 1
// 0 1
// 0 0
impl Direction {
    pub fn from(old: (u8, u8), new: (u8, u8)) -> Option<Self> {
        use Direction::*;
        match (old, new) {
            ((0, 0), (0, 1)) => Some(CW),
            ((0, 1), (1, 1)) => Some(CW),
            ((1, 1), (1, 0)) => Some(CW),
            ((1, 0), (0, 0)) => Some(CW),
            ((1, 0), (1, 1)) => Some(CCW),
            ((1, 1), (0, 1)) => Some(CCW),
            ((0, 1), (0, 0)) => Some(CCW),
            ((0, 0), (1, 0)) => Some(CCW),
            _ => None,
        }
    }
}

#[derive(Copy,Clone,Debug,Eq,PartialEq)]
enum Pin {
    A,
    B,
}

type InternalStream<S> where S: Stream<Item = u8> + 'static =
    Box<Stream<Item = Direction, Error = S::Error> + 'static>;

/// A Stream of rotation directions decoded from a rotary encoder.
pub struct RotaryEncoder<S>(InternalStream<S>) where S: Stream<Item = u8> + 'static;

impl<S> RotaryEncoder<S>
    where S: Stream<Item = u8>
{
    pub fn new(a: S, b: S) -> Self {
        let mut values = (2, 2);
        RotaryEncoder(Box::new(a.map(|v| (Pin::A, v))
            .select(b.map(|v| (Pin::B, v)))
            .filter_map(move |(p, val)| {
                let previous = values;
                match p {
                    Pin::A => values.0 = val,
                    Pin::B => values.1 = val,
                }
                Direction::from(previous, values)
            })))
    }
}

impl<S> Stream for RotaryEncoder<S>
    where S: Stream<Item = u8> + 'static
{
    type Item = Direction;
    type Error = S::Error;

    fn poll(&mut self) -> Poll<Option<Direction>, S::Error> {
        self.0.poll()
    }
}
