use serde::{Serialize, Deserialize};

use fls::prelude::FLSMsg;

#[derive(Serialize, Deserialize, Clone)]
pub struct Controller
{
    pub btns : Button,
    pub dpad : DPad,
    pub stick : JoyStick
}
impl FLSMsg for Controller {
    
}
impl Controller {
    pub fn new()->Controller
    {
        Controller { btns: Button::new(), dpad: DPad::new(), stick: JoyStick::new() }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Button{
    pub  circle:bool,
    pub  triangle:bool,
    pub  cube:bool,
    pub  cross:bool,
    pub  r1:bool,
    pub  r2:bool,
    pub  l1:bool,
    pub  l2:bool,
}
impl Button {
    pub fn new()->Self
    {
        Button { circle: false, triangle: false, cube: false, cross: false, r1: false, r2: false, l1: false, l2: false }
    }
}

impl FLSMsg for Button {}
#[derive(Serialize, Deserialize, Clone)]
pub struct DPad{
    pub  up:bool,
    pub  down:bool,
    pub  right:bool,
    pub  left:bool,
}
impl DPad {
    pub fn new()->Self
    {
        DPad { up: false, down: false, right: false, left: false }
    }
}
impl FLSMsg for DPad {}

#[derive(Serialize, Deserialize, Clone)]
pub struct JoyStick{
    pub  left_x:f32,
    pub  left_y:f32,
    pub  right_x:f32,
    pub  right_y:f32,
}
impl JoyStick {
    pub fn new()->Self
    {
        JoyStick { left_x: 0.0, left_y: 0.0, right_x: 0.0, right_y: 0.0 }
    }
}
impl FLSMsg for JoyStick {}