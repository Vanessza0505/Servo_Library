#![no_std]

use embedded_hal::pwm::SetDutyCycle;
use embassy_time::{Duration, Timer};
use esp_hal::ledc::channel::Channel;
use esp_hal::ledc::HighSpeed;


pub struct Servo {
    channel: Channel<'static, HighSpeed>,
    min_duty: u32,
    max_duty: u32,
    max_angle: u32,
    angle: u32,
    open_time: u64
}

impl Servo {
    pub fn new(
        channel: Channel<'static, HighSpeed>,
        min_duty: u32,
        max_duty: u32,
        max_angle: u32,
        angle: u32,
        open_time: u64
    ) -> Self {
        Self {
            channel,
            min_duty,
            max_duty,
            max_angle,
            angle,
            open_time
        }
    }

    //this function communicate the hardwear and actually rotate the servo
    pub fn rotate(&mut self, angle: u32){
        let duty_gap = self.max_duty - self.min_duty;
        let duty = self.min_duty + (angle * duty_gap) / self.max_angle;
        self.channel.set_duty_cycle(duty as u16);
    }

    //this is open the boat door
    pub async fn open(&mut self){
        for deg in 0..=self.angle {
            self.rotate(deg);
            Timer::after(Duration::from_millis(15)).await;
        }
    }


    // this is close the boat door
    pub async fn close(&mut self){
        for deg in (0..=self.angle).rev() {
            self.rotate(deg);
            Timer::after(Duration::from_millis(15)).await;
        }
    }


    // this is the feeding function when the fisherman click the button to feed the fish this made the door to open with the servo helping and it will close
    pub async fn feed(&mut self){
        self.open().await;
        Timer::after(Duration::from_secs(self.open_time)).await; //opening time 
        self.close().await;
    }
}
