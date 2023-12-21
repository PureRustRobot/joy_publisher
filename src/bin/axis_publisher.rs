extern crate hidapi;
extern crate dualshock4;

use hidapi::HidApi;
use serde_json;

use zenoh::{
    config::Config,
    prelude::r#async::*,
};

use joy_publisher::get_axis;
use async_std;

#[async_std::main]
async fn main() {
    let api = HidApi::new().expect("Failed to create HID API instance.");
    let controller = dualshock4::get_device(&api).expect("Failed to open device");

    let session = zenoh::open(Config::default()).res().await.unwrap();
    let topic = "joy_publisher/axis".to_string();
    let publisher = session.declare_publisher(&topic).res().await.unwrap();

    loop {
        match dualshock4::read(&controller) {
            Ok(data)=>
            {
                let axis = get_axis(data);
                let buf = serde_json::to_string(&axis).unwrap();

                publisher.put(buf.clone()).res().await.unwrap();
                println!("{}", buf);
            }
            Err(e)=>
            {
                println!("[ERROR]:{}", e);
            }
        }
    }
}