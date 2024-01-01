use v4l::buffer::Type;
use v4l::io::traits::CaptureStream;
use v4l::prelude::*;
use std::{thread, time};
use std::collections::HashMap;
use std::process::Command;
use regex::Regex;

fn mean(list: &[u8]) -> f64
{
    let mut sum: i32 = 0i32;
    for i in 0..list.len() {
        sum += list[i] as i32;
    }
    f64::from(sum) / (list.len() as f64)
}

fn max(list: &[u8]) -> u8
{
    let mut max_num = &list[0];
    for i in 0..list.len() {
        if *max_num < list[i] {
            max_num = &list[i];
        }
    }
    max_num.clone()
}

fn mode_map(m: &HashMap<u8, u32>) -> Option<&u8> {
    let mut max_num = None;
    let mut max_count = 0u32;

    for (k, &v) in m.iter() {
        if v > max_count {
            max_count = v;
            max_num = Some(k);
        }
    }

    max_num
}

fn mode(list: &[u8]) -> Option<u8> {
    let mut m: HashMap<u8, u32> = HashMap::new();

    for &it in list.iter() {
        let count = m.entry(it).or_insert(0);
        *count += 1;
    }

    mode_map(&m).cloned()
}

pub fn middle_half_average(values: &[u8]) -> Option<f64> {
    // Ensure the input list is not empty
    if values.is_empty() {
        return None;
    }

    // Sort the values
    let mut sorted_values = values.to_vec();
    sorted_values.sort();

    // Calculate the index range for the upper quartile
    let len = sorted_values.len();
    let start_index = (1 * len) / 4;
    let end_index = (3 * len) / 4;

    // Calculate the sum of values in the upper quartile
    let sum: u32 = sorted_values.iter().skip(start_index).map(|&x| u32::from(x)).sum();

    // Calculate the average
    let count = (end_index - start_index + 1) as f64;
    let average = f64::from(sum) / count;

    Some(average)
}

pub fn create_brightness_value(offset: i32, data: &[u8]) -> u8
{
    (
        (
            -130.5f64
            + (offset as f64)
            + (middle_half_average(&data).unwrap() / 2.55f64 * 4.0f64)
            + (mean(&data) / 2.55f64 * 3.0f64)
        )
    ).clamp(0f64, 99f64).round() as u8
}

pub fn get_curr_brightness() -> u32 {
    let output = Command::new("brightnessctl")
    .arg("i")
    .output()
    .map_err(|e| format!("Failed to execute command: {}", e)).unwrap();

    if output.status.success() {
        let re = Regex::new(r"Current brightness.*\((\d+)%\)").expect("Invalid regex pattern");

        let output_str = String::from_utf8_lossy(&output.stdout);
        for caps in re.captures_iter(&output_str) {
            if let Some(percentage) = caps.get(1) {
                let percentage_value: u32 = percentage.as_str().parse().unwrap();
                return percentage_value;
            }
        }
        panic!();
    } else {
        panic!();
    }
}

fn set_brigtness_command(b: &u8)
{
    let command = Command::new("brightnessctl")
    .arg("set")
    .arg(format!("{}%", &b))
    .spawn();

    match command {
        Ok(mut child) => {
            // Wait for the command to finish and collect the exit status
            let status = child.wait().expect("Failed to wait for command");

            if status.success() {
                // println!("Command executed successfully");
            } else {
                eprintln!("Command failed with exit code: {:?}", status.code());
            }
        },
        Err(e) => eprintln!("Error spawning command: {:?}", e)
    }
}

pub fn set_brightness_with_animation(old_brightness: u8, new_brightness: u8, anim_frames: u16, anim_time: u64)
{
    for frame_num in 0..anim_frames - 1 {
        let i: f32 = frame_num as f32;
        let range: f32 = (new_brightness as f32) - (old_brightness as f32);
        let delta: f32 = i / (anim_frames as f32) * range; //+ (old_brightness as f32);
        let mut value: i16 = (delta.round() as i16) + (old_brightness as i16);
        if value < 0 {
            value = 0;
        }
        let intermediate_brightness_value: u8 = value as u8;

        set_brigtness_command(&intermediate_brightness_value);

        thread::sleep(time::Duration::from_millis( anim_time / (anim_frames as u64) ));
    }
    set_brigtness_command(&new_brightness);

}
