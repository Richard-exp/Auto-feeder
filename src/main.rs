use std::error::Error;
use std::thread;
use std::time::Duration;

use reqwest::{header::CONTENT_LENGTH, Response};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio;

const URL: &str = "http://vps.dowy.org/";

use rppal::gpio::{Gpio, OutputPin};

// Gpio uses BCM pin numbering.
const IN1_GPIO: u8 = 18;
const IN2_GPIO: u8 = 23;
const IN3_GPIO: u8 = 24;
const IN4_GPIO: u8 = 25;

const TIME: u64 = 20;

struct ServoPins {
    IN1: OutputPin,
    IN2: OutputPin,
    IN3: OutputPin,
    IN4: OutputPin,
}

async fn basic(break_point: &str) -> Result<Response, Box<dyn std::error::Error>> {
    //let client = reqwest::Client::new();
    let resp = reqwest::get(format!("{URL}{break_point}")).await?;
    println!("body = {:?}", resp);
    Ok(resp)
}

async fn get_json(
    data: &Value,
    client: &reqwest::Client,
    break_point: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = json!({"user_id": data.as_i64().unwrap()});

    let res = client
        .get(format!("{URL}{break_point}"))
        .json(&data)
        .send()
        .await?;
    // let ser_res = res.text().await?;
    println!("res = {:?}", res);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut user_id: Value = json!(1);

    loop {
        let res = basic("users-data-get").await?;
        let ser_res = res.json::<Value>().await?;

        let res_len = ser_res.as_array().unwrap().len();
        if let Some(feed_now) = ser_res
            .get(res_len - 1)
            .and_then(|v| v.get("user_info_dict"))
            .and_then(|v| v.get("profile_info"))
            .and_then(|v| v.get("feeder_info"))
            .and_then(|v| v.get("feed_now"))
        {
            if feed_now == true {
                feed_action();

                let feed_param = ser_res
                    .get(res_len - 1)
                    .and_then(|v| v.get("user_info_dict"))
                    .and_then(|v| v.get("profile_info"))
                    .and_then(|v| v.get("feeder_info"))
                    .and_then(|v| v.get("manual_feeding"));

                user_id = feed_param.unwrap().get("user_id").unwrap().clone();
                println!("feed_param = {:?}", feed_param);
                let res = get_json(&user_id, &client, "manual-feeding").await?;
            }
        } else {
            eprintln!("Field not found");
        }
    }
    Ok(())
}

impl ServoPins {
    fn step1(&mut self) {
        self.IN4.set_high();
        thread::sleep(Duration::from_millis(TIME));
        self.IN4.set_low();
    }
    fn step3(&mut self) {
        self.IN3.set_high();
        thread::sleep(Duration::from_millis(TIME));
        self.IN3.set_low();
    }
    fn step5(&mut self) {
        self.IN2.set_high();
        thread::sleep(Duration::from_millis(TIME));
        self.IN2.set_low();
    }
    fn step7(&mut self) {
        self.IN1.set_high();
        thread::sleep(Duration::from_millis(TIME));
        self.IN1.set_low();
    }
    fn left(&mut self, steps: i64) {
        for _ in 0..steps {
            self.step1();
            self.step3();
            self.step5();
            self.step7();
        }
        self.power_off();
    }
    fn right(&mut self, steps: i64) {
        for _ in 0..steps {
            self.step7();

            self.step3();
            self.step5();
            self.step1();
        }
        self.power_off();
    }

    fn power_off(&mut self) {
        self.IN1.is_set_low();
        self.IN2.is_set_low();
        self.IN3.is_set_low();
        self.IN4.is_set_low();
    }
    fn move_motor(&mut self, steps: i64) {
        for _ in 0..steps {
            self.left(2);
            self.right(6);
        }
    }
}

fn feed_action() {
    let mut pins: ServoPins = {
        ServoPins {
            IN1: (Gpio::new().unwrap().get(IN1_GPIO).unwrap().into_output()),
            IN2: (Gpio::new().unwrap().get(IN2_GPIO).unwrap().into_output()),
            IN3: (Gpio::new().unwrap().get(IN3_GPIO).unwrap().into_output()),
            IN4: (Gpio::new().unwrap().get(IN4_GPIO).unwrap().into_output()),
        }
    };
    pins.move_motor(30);
}
