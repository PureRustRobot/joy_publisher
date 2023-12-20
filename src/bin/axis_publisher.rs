extern crate hidapi;
extern crate dualshock4;

use hidapi::HidApi;
use serde_json;

use zenoh::{
    config::Config,
    prelude::r#async::*,
};

use joy_publisher::get_axis;

fn main() {
    let api = HidApi::new().expect("Failed to create HID API instance.");
    let controller = dualshock4::get_device(&api).expect("Failed to open device");

    let session = zenoh::open(Config::default()).res().await.unwrap();
    let topic = "/joy".to_string();
    let publisher = session.declare_publisher(&topic).res().await.unwrap();

    loop {
        match dualshock4::read(&controller) {
            Ok(data)=>
            {
                let axis = get_axis(data);
                let buf = serde_json::to_string(&data).unwrap();

                publisher.put(buf).res().await.unwrap();
            }
            Err(e)=>
            {
                println!("[ERROR]:{}", e);
            }
        }

        println!("{:?}",data);
    }
}