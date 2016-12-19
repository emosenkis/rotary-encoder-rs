extern crate futures;
extern crate rotary_encoder;
extern crate sysfs_gpio;
extern crate tokio_core;

use std::env;

use futures::Stream;
use rotary_encoder::RotaryEncoder;
use sysfs_gpio::{Edge, Pin};
use tokio_core::reactor::Core;

fn main() {
    let mut pins = env::args()
        .skip(1)
        .take(2)
        .map(|a| Pin::new(a.parse().expect("Pins must be specified as integers")));
    let a = pins.next().expect("Please provide two pin numbers");
    let b = pins.next().expect("Please provide two pin numbers");
    a.with_exported(|| {
            b.with_exported(|| {
                let mut l = try!(Core::new());
                let handle = l.handle();
                a.set_direction(sysfs_gpio::Direction::In)?;
                b.set_direction(sysfs_gpio::Direction::In)?;
                a.set_edge(Edge::BothEdges)?;
                b.set_edge(Edge::BothEdges)?;
                let a_stream = a.get_value_stream(&handle)?;
                let b_stream = b.get_value_stream(&handle)?;
                let dir_stream = RotaryEncoder::new(a_stream, b_stream);
                l.run(dir_stream.for_each(|dir| Ok(println!("{:?}", dir))))
            })
        })
        .unwrap();
}
