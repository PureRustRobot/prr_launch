use async_std;
use zenoh::Error;

use joy_publisher::joy_publisher;
use command_controller::gamecon_to_cmd_vel;
use motor_controller::cmd_vel_to_wheel;
use udp_bridge::wheel_bridge;

#[async_std::main]
async fn main()->Result<(), Error>
{
    let controller_task = async_std::task::spawn(joy_publisher("joy_publisher", "game_con", "ble"));

    let cmd_convert_task = async_std::task::spawn(gamecon_to_cmd_vel("gamecon_to_cmd_vel", "game_con", "cmd_vel/wheel", false));

    let cmd_to_wheel = async_std::task::spawn(cmd_vel_to_wheel("cmd_vel_to_wheel", "cmd_vel/wheel", "motor/wheel", 1.0, false));

    let wheel_to_esp32 = async_std::task::spawn(wheel_bridge(
        "to_esp32", 
        "motor/wheel", 
        "192.168.4.2:8080", 
        "192.168.4.1:10000", 
        true));

    controller_task.await?;
    cmd_convert_task.await?;
    cmd_to_wheel.await?;
    wheel_to_esp32.await?;

    Ok(())
}
