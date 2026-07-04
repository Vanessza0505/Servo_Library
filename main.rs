#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;

use esp_hal::clock::CpuClock;
use esp_hal::timer::timg::TimerGroup;
use esp_println as _;
use esp_hal::ledc::channel::ChannelIFace;
use esp_hal::ledc::timer::TimerIFace;
use esp_hal::ledc::{HighSpeed, Ledc, channel, timer};
use esp_hal::time::Rate;
use esp_hal::delay::Delay;
use embedded_hal::pwm::SetDutyCycle;

use esp32_servo_controller::{Servo};

use static_cell::StaticCell;
use esp_hal::ledc::channel::config::PinConfig;



#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // generator version: 0.3.1

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timer0 = TimerGroup::new(peripherals.TIMG1);
    esp_hal_embassy::init(timer0.timer0);

    info!("Embassy initialized!");

    // TODO: Spawn some tasks
    let _ = spawner;



    let servo_pin = peripherals.GPIO12;
    let ledc = Ledc::new(peripherals.LEDC);

    static HSTIMER0: StaticCell<esp_hal::ledc::timer::Timer<'static, HighSpeed>> = StaticCell::new();

    let mut hstimer0 = ledc.timer::<HighSpeed>(timer::Number::Timer0);
    hstimer0
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty14Bit,
            clock_source: timer::HSClockSource::APBClk,
            frequency: Rate::from_hz(50),
        })
        .unwrap();

    let hstimer0: &'static _ = HSTIMER0.init(hstimer0);

    let mut channel0 = ledc.channel(channel::Number::Channel0, servo_pin);
    channel0
        .configure(channel::config::Config {
            timer: hstimer0,
            duty_pct: 0,
            pin_config: PinConfig::PushPull,
        })
        .unwrap();

    let max_duty_cycle = channel0.max_duty_cycle() as u32;
    let min_duty = (25 * max_duty_cycle) / 1000;
    let max_duty = (125 * max_duty_cycle) / 1000;

    let delay = Delay::new();

    
    let mut servo = Servo::new(channel0, min_duty, max_duty,180, 90);

    servo.feed(&delay);
    delay.delay_millis(5000);

    loop {}

}
