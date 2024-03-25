use async_std;
use joy_publisher::joy_publisher;
use zenoh::Error;

#[async_std::main]
async fn main()->Result<(), Error>
{
    let task = async_std::task::spawn(joy_publisher("joy_publisher", "game_con", "ble"));

    task.await?;

    Ok(())
}