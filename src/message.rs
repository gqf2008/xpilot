use crate::driver::{Accel, Barometer, Compass, Distance, EulerAngle, Gps, Gyro, Quaternion};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Quaternion(Quaternion),
    // 欧拉角
    YawPitchRoll(EulerAngle),

    // 角度
    Gyro(Gyro),
    // 角加速度
    Accel(Accel),
    // 罗盘
    Compass(Compass),
    //气压计
    Barometer(Barometer),

    Distance(Distance),
    Gps(Gps),
    //控制信号
    Control(Signal),
}

#[derive(Debug, Clone, Copy)]
pub enum Signal {
    Led {},
    Motor {},
    Servo {},
}
