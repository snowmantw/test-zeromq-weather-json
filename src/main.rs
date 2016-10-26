#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

/// Weather update server
/// Binds PUB socket to tcp://*:5556 and ipc://weather.ipc
/// Publishes random weather updates


extern crate serde;
extern crate serde_json;
extern crate rand;
extern crate zmq;

use rand::Rng;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Weather {
    zipcode: u16,
    temperature: i16,
    relhumidity: i8
}

fn main() {
    println!(" -- Server launched -- ");

    let mut context = zmq::Context::new();
    let mut publisher = context.socket(zmq::PUB).unwrap();

    assert!(publisher.bind("tcp://*:5556").is_ok());
    assert!(publisher.bind("ipc://weather.ipc").is_ok());

    let mut rng = rand::weak_rng();

    loop {

        let weather = Weather {
            zipcode: rng.gen_range(0, 10000),
            temperature: rng.gen_range(-80, 135),
            relhumidity: rng.gen_range(10, 60)
        };

        let update = serde_json::to_string(&weather).unwrap();
        publisher.send_str(&update, 0).unwrap();
    }

    // note: destructors mean no explicit cleanup necessary
}
