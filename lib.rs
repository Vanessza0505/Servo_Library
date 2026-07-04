#![no_std]

use embedded_hal::pwm::SetDutyCycle;
use esp_hal::delay::Delay;
use esp_hal::ledc::channel::Channel;
use esp_hal::ledc::HighSpeed;

pub enum ServoError {
    InvalidAngle,
    CommunicationError,
}


pub struct Servo {
    channel: Channel<'static, HighSpeed>,
    min_duty: u32,
    max_duty: u32,
    max_angle: u32,
    angle: u32,
}

impl Servo {
    pub fn new(
        channel: Channel<'static, HighSpeed>,
        min_duty: u32,
        max_duty: u32,
        max_angle: u32,
        angle: u32,
    ) -> Self {
        Self {
            channel,
            min_duty,
            max_duty,
            max_angle,
            angle,
        }
    }


    pub fn rotate(&mut self, angle: u32) -> Result<(), ServoError> {
        if angle > self.max_angle {
            return Err(ServoError::InvalidAngle);
        }

        let duty_gap = self.max_duty - self.min_duty;
        let duty = self.min_duty + (angle * duty_gap) / self.max_angle;

        self.channel
            .set_duty_cycle(duty as u16)
            .map_err(|_| ServoError::CommunicationError)
    }

    
    pub fn feed(&mut self, delay: &Delay) -> Result<(), ServoError> {
        for deg in 0..=self.angle {
            self.rotate(deg);
            delay.delay_millis(15);
        }
        for deg in (0..=self.angle).rev() {
            self.rotate(deg);
            delay.delay_millis(15);
        }
        Ok(())
    }
}
