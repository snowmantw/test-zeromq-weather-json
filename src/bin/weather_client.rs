#![crate_name = "weather_client"]
#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

/*!
 * Weather update client
 * Connects SUB socket to tcp://localhost:5556
 * Collects weather updates and find avg temp in zipcode
 */
extern crate serde;
extern crate serde_json;

extern crate zmq;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Weather {
    zipcode: u16,
    temperature: i16,
    relhumidity: i8
}

fn main() {
    println!("Collecting updates from weather server...");

    let mut context = zmq::Context::new();
    let mut subscriber = context.socket(zmq::SUB).unwrap();
    assert!(subscriber.connect("tcp://localhost:5556").is_ok());
    assert!(subscriber.set_subscribe("".as_bytes()).is_ok());

    loop {
        // NOTE: May get stuck if it cannot get the message
        let str_weather = subscriber.recv_string(0).unwrap().unwrap();
        let weather: Weather = serde_json::from_str(&str_weather).unwrap();
        println!("Temperature for zipcode '{}' was {}F", weather.zipcode, weather.temperature);
    }
}
