#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::timer::timg::TimerGroup;
use esp_println as _;
use esp_hal::ledc::channel::ChannelIFace;
use esp_hal::ledc::timer::TimerIFace;
use esp_hal::ledc::{HighSpeed, Ledc, channel, timer};
use esp_hal::time::Rate;
use embedded_hal::pwm::SetDutyCycle;
use esp32_servo_controller::{Servo};
use esp_hal::ledc::channel::config::PinConfig;



#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timer0 = TimerGroup::new(peripherals.TIMG1);
    esp_hal_embassy::init(timer0.timer0);

    info!("Embassy initialized!");

    let _ = spawner;



    let servo_pin = peripherals.GPIO12;
    let ledc = Ledc::new(peripherals.LEDC);

    
    let mut hstimer0 = ledc.timer::<HighSpeed>(timer::Number::Timer0);
    hstimer0
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty12Bit,
            clock_source: timer::HSClockSource::APBClk,
            frequency: Rate::from_hz(50),
        })
        .unwrap();

    let mut channel0 = ledc.channel(channel::Number::Channel0, servo_pin);
    channel0
        .configure(channel::config::Config {
            duty_pct: 10,
            pin_config: PinConfig::PushPull,
            timer: &hstimer0,
        })
        .unwrap();
    
    let max_duty_cycle = channel0.max_duty_cycle() as u32;
    let min_duty = (25 * max_duty_cycle) / 1000;
    let max_duty = (125 * max_duty_cycle) / 1000;


    
    let mut servo = Servo::new(channel0, min_duty, max_duty,180, 170, 1);
    

    // servo.open().await;
    // servo.close().await;

    servo.feed().await;
   

    loop {}

}
