use rust_gpiozero::{InputDevice, OutputDevice};
use std::{thread::sleep, time::Instant};

const MAX_COUNT: usize = 3;
const DISTANCE: f64 = 30.;

const SENS_SLEEP: u64 = 260; //300

fn sens(trigger_id: u8, echo: u8) -> std::thread::JoinHandle<f64> {
    let builder = std::thread::Builder::new();
    builder
        .spawn(move || {
            let mut trigger = OutputDevice::new(trigger_id);
            let echo = InputDevice::new(echo);
            trigger.on();
            sleep(std::time::Duration::from_secs_f64(5e-6));
            trigger.off();

            let mut start = Instant::now();
            let mut end = Instant::now();
            while !echo.is_active() {
                start = Instant::now();
                if (start - end).as_secs_f64() + (start - end).as_micros() as f64 / 1e6 > 1. {
                    return 1337.;
                }
            }
            while echo.is_active() {
                end = Instant::now();
            }
            let duration = end - start;
            let distance = duration.as_secs_f64() * 17150.;
            sleep(std::time::Duration::from_millis(SENS_SLEEP));
            distance
        })
        .unwrap()
}

fn main() {
    let mut counter = 0usize;
    let mut activated = false;

    minreq::get("http://172.20.10.4:5000/untrigger")
        .send()
        .unwrap();

    loop {
        let h1 = sens(23, 24);
        let h2 = sens(20, 21);

        println!("counter: {}", counter);

        let (distance_x, distance_y) = (h1.join().unwrap(), h2.join().unwrap());
        if distance_x <= DISTANCE || distance_y <= DISTANCE {
            if counter < MAX_COUNT {
                counter += 1;
            }
            if !activated && counter >= MAX_COUNT {
                minreq::get("http://172.20.10.4:5000/trigger")
                    .send()
                    .unwrap();
                activated = true;
                counter = MAX_COUNT;
            }
        } else {
            if counter > 0 {
                counter -= 1;
            } else if activated == true {
                minreq::get("http://172.20.10.4:5000/untrigger")
                    .send()
                    .unwrap();
                activated = false;
            }
        }
        println!("distance_x: {}, distance_y: {}", distance_x, distance_y);
    }
}
