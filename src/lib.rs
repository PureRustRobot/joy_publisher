use z_interface::dual_shock_4::{self, Axis, Buttons};
use dualshock4::Dualshock4Data;

pub fn get_axis(data:Dualshock4Data)->dual_shock_4::Axis
{
    Axis{
        joy_left_x:data.analog_sticks.left.x as f32,
        joy_left_y:data.analog_sticks.left.y as f32,
        joy_right_x:data.analog_sticks.right.x as f32,
        joy_right_y:data.analog_sticks.right.y as f32,
    }
}

pub fn get_button(data:Dualshock4Data)->dual_shock_4::Buttons
{
    Buttons{
        joy_left:get_value(data.buttons.left_stick.pressed),
        joy_right:get_value(data.buttons.right_stick.pressed),
        circle:get_value(data.buttons.circle.pressed),
        cross:get_value(data.buttons.x.pressed),
        square:get_value(data.buttons.square.pressed),
        triangle:get_value(data.buttons.triangle.pressed),
        up:get_value(data.buttons.dpad_up.pressed),
        down:get_value(data.buttons.dpad_down.pressed),
        right:get_value(data.buttons.dpad_right.pressed),
        left:get_value(data.buttons.dpad_left.pressed),
        _l1_:get_value(data.buttons.l1.pressed),
        _l2_:get_value(data.buttons.l2.pressed),
        _r1_:get_value(data.buttons.r1.pressed),
        _r2_:get_value(data.buttons.r2.pressed),
    }
}

fn get_value(data:bool)->f32
{
    if data == true
    {
        return 1.0
    }
    else {
        return 0.0;
    }
}