use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio, OutputPin};

// Gpio uses BCM pin numbering. 
const IN1_GPIO: u8 = 4;
const IN2_GPIO: u8 = 17;
const IN3_GPIO: u8 = 23;
const IN4_GPIO: u8 = 24;
const TIME: u64 = 1;

struct Servo_pins {
    IN1: OutputPin,
    IN2: OutputPin,
    IN3: OutputPin,
    IN4: OutputPin,
}

impl Servo_pins {
    fn step1(&mut self) {
        self.IN4.set_high();
        thread::sleep(Duration::from_millis(TIME));
        self.IN4.set_low();
    }
    fn step2(&mut self) {
        self.IN4.set_high();
        self.IN3.set_high();
        thread::sleep(Duration::from_millis(TIME));
        self.IN4.set_low();
        self.IN3.set_high();
    }
    fn step3(&mut self) {
        self.IN3.set_high();
        thread::sleep(Duration::from_millis(TIME));
        self.IN3.set_low();
    }
    fn step4(&mut self) {
        self.IN3.set_high();
        self.IN2.set_high();
        thread::sleep(Duration::from_millis(TIME));
        self.IN3.set_low();
        self.IN2.set_low();
    }
    fn step5(&mut self) {
        self.IN2.set_high();
        thread::sleep(Duration::from_millis(TIME));
        self.IN2.set_low();
    }
    fn step6(&mut self) {
        self.IN2.set_high();
        self.IN1.is_set_high();
        thread::sleep(Duration::from_millis(TIME));
        self.IN2.set_low();
        self.IN1.set_low();

    }
    fn step7(&mut self) {
        self.IN1.set_high();
        thread::sleep(Duration::from_millis(TIME));
        self.IN1.set_low();
    }
    fn step8(&mut self) {
        self.IN1.set_high();
        self.IN4.set_high();
        thread::sleep(Duration::from_millis(TIME));
        self.IN1.set_low();
        self.IN4.set_low();
    }
    fn left(&mut self, steps: i64){
        for _ in 0..steps {
            self.step1();
            self.step2();
            self.step3();
            self.step4();
            self.step5();
            self.step6();
            self.step7();
            self.step8();
        }
    }
    fn right(&mut self, steps: i64){
        for _ in 0..steps {
            self.step8();
            self.step7();
            self.step6();
            self.step5();
            self.step4();
            self.step3();
            self.step2();
            self.step1();
        }
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let mut pins: Servo_pins = {
        Servo_pins { 
            IN1: (Gpio::new()?.get(IN1_GPIO)?.into_output()), 
            IN2: (Gpio::new()?.get(IN2_GPIO)?.into_output()), 
            IN3: (Gpio::new()?.get(IN3_GPIO)?.into_output()), 
            IN4: (Gpio::new()?.get(IN4_GPIO)?.into_output()) }
    }; 
    pins.left(512);

    Ok(())
}



