use crate::math::{get_curr_brightness, create_brightness_value, middle_half_average, set_brightness_with_animation};

use v4l::buffer::Type;
use v4l::io::traits::CaptureStream;
use v4l::prelude::*;
use std::{thread, time};
use std::process::Command;
use std::sync::{Arc, Mutex};

use crate::daemon::Daemon;

fn set_brightness(new_brightness: &u8)
{
    let command = Command::new("brightnessctl")
        .arg("set")
        .arg(format!("{}%", &new_brightness))
        .spawn();

    match command {
        Ok(mut child) => {
            // Wait for the command to finish and collect the exit status
            let status = child.wait().expect("Failed to wait for command");

            if !status.success() {
                eprintln!("Command failed with exit code: {:?}", status.code());
            }
        },
        Err(e) => eprintln!("Error spawning brightnessctl command: {:?}", e)
    }
}

pub fn setup_camera_daemon() -> Arc<Mutex<Daemon>>
{
    // make two threads on for reading and one for writing
    // have the camera close and wait if another application is using it

    // TODO make the expects not crash and instead just loop again
    let handler = thread::spawn(|| {
        // (brightness value, camera value)
        let mut hist: Vec<(i32, u8)> = vec![(10i32, 150u8); 5];
        let mut initialized = false;
        let mut offset: i32 = 0i32;
        // let mut offset: f64 = 0f64;
        loop {
            match Device::new(0) {
                Ok(mut dev) => {
                    thread::sleep(time::Duration::from_millis(500));
                    let mut stream =
                    MmapStream::with_buffers(&mut dev, Type::VideoCapture, 4).expect("Failed to create buffer stream");
                    let (buf, _meta) = stream.next().unwrap();

                    let bri = get_curr_brightness();

                    offset = match initialized {
                        true => offset as i32 + bri as i32 - hist[0].0 as i32,
                        false => 0
                    };

                    hist.pop();

                    let mut avg_a: f64 = 0f64;
                    let mut avg_b: f64 = 0f64;
                    for (k, v) in hist.iter().enumerate() {
                        let (a, b) = v;
                        avg_a += *a as f64;
                        avg_b += *b as f64;
                    }
                    avg_a /= hist.len() as f64;
                    avg_b /= hist.len() as f64;

                    let new_brightness: u8 = create_brightness_value(offset, &buf);
                    hist.insert(0, (new_brightness as i32, middle_half_average(&buf).unwrap() as u8));

                    if initialized {
                        set_brightness_with_animation(bri as u8, new_brightness, 40u16, 1150u64);
                    }
                    initialized = true;

                    thread::sleep(time::Duration::from_secs(3));
                },
                _ => {
                    eprintln!("retrying device access in 5 secs");
                    thread::sleep(time::Duration::from_secs(5));
                }
            }
        }
    });

    // handler.join().unwrap();

    return Arc::new(Mutex::new(Daemon::new()));
}
