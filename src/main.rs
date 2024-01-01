use v4l::buffer::Type;
use v4l::io::traits::CaptureStream;
use v4l::prelude::*;
use std::{thread, time};
use std::collections::HashMap;
use std::process::Command;
use regex::Regex;

mod brightness_calc;
use brightness_calc::{get_curr_brightness, create_brightness_value, middle_half_average, set_brightness_with_animation};

// TODO create a struct for the camera to abstract parameters to config files
// TODO integrate with GUI for control and settings views

fn main() {

    // make two threads on for reading and one for writing
    // have the camera close and wait if another application is using it

    // TODO make the expects not crash and instead just loop again
    let handler = thread::spawn(|| {
        // (brightness value, camera value)
        let mut hist: Vec<(u32, u8)> = vec![(10u32, 150u8); 5];
        let mut initialized = false;
        let mut offset: i32 = 0i32;
        loop {
            match Device::new(0) {
                Ok(mut dev) => {
                    thread::sleep(time::Duration::from_millis(500));
                    let mut stream =
                    MmapStream::with_buffers(&mut dev, Type::VideoCapture, 4).expect("Failed to create buffer stream");
                    let (buf, _meta) = stream.next().unwrap();

                    let bri = get_curr_brightness();

                    offset += bri as i32 - hist[0].0 as i32;
                    offset *= initialized as i32;

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
                    hist.insert(0, (new_brightness as u32, middle_half_average(&buf).unwrap() as u8));

                    if initialized {
                        let curr_brightness: u8 = bri as u8;
                        set_brightness_with_animation(curr_brightness, new_brightness, 30, 1250);
                    }
                    else {
                        initialized = true;
                    }

                    thread::sleep(time::Duration::from_secs(1));
                },
                _ => {
                    eprintln!("retrying device access in 5 secs");
                    thread::sleep(time::Duration::from_secs(5));
                }
            }
        }
    });

    handler.join().unwrap();
}
