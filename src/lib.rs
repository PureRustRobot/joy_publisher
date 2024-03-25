use zenoh::{
    config::Config,
    prelude::r#async::*,
    Error
};

use prr_utils::logger::log_info;
use dualshock_driver::{DualShock4Driver, BLE, SERIAL};
use prr_msgs::msg::GameCon;

pub async fn joy_publisher(name:&str, pub_topic:&str, mode:&str)->Result<(), Error>
{
    let session = zenoh::open(Config::default()).res().await.unwrap();
    let publisher = session.declare_publisher(pub_topic).res().await.unwrap();

    let mut driver = match mode {
        "ble"=>{DualShock4Driver::new(BLE).unwrap()},
        "serial"=>{DualShock4Driver::new(SERIAL).unwrap()},
        _=>{DualShock4Driver::new(BLE).unwrap()}
    };

    log_info(name, format!("Start {}. topic:{}, mode:{}", name, pub_topic, mode));

    loop {
        let con = driver.read().await.unwrap();

        let msg = GameCon{
            left_x:con.sticks.left_x,
            left_y:con.sticks.left_y,
            right_x:con.sticks.right_x,
            right_y:con.sticks.right_y,
            circle:con.btns.circle,
            cross:con.btns.cross,
            cube:con.btns.cube,
            triangle:con.btns.triangle,
            up_key:con.dpad.up_key,
            down_key:con.dpad.down_key,
            right_key:con.dpad.right_key,
            left_key:con.dpad.left_key,
            r1:con.btns.r1,
            r2:con.btns.r2,
            l1:con.btns.l1,
            l2:con.btns.l2
        };

        let _ = publisher.put(GameCon::serialize(&msg)).res().await.unwrap();
    }
}