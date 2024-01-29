mod camera;
use camera::{setup_camera_daemon};

mod math;

mod daemon;
use daemon::{Daemon};

pub mod gui;
use gui::{launch_window};

mod signals;

use std::sync::{Arc, Mutex};

// mod my_qt_object;
// TODO create a struct for the camera to abstract parameters to config files
// TODO integrate with GUI for control and settings views

fn main() {
    let daemon: Arc<Mutex<Daemon>> = setup_camera_daemon();
    launch_window(daemon.clone());
}
     max_num = &list[i];
        }
    }
    max_num
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

fn middle_half_average(values: &[u8]) -> Option<f64> {
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
    // let end_index = len - 1;

    // Calculate the sum of values in the upper quartile
    let sum: u32 = sorted_values.iter().skip(start_index).map(|&x| u32::from(x)).sum();

    // Calculate the average
    let count = (end_index - start_index + 1) as f64;
    let average = f64::from(sum) / count;

    Some(average)
}

fn create_brightness_value(offset: f64, data: &[u8]) -> u8
{
    (
        (
            -130.5f64
            + offset
            + (middle_half_average(&data).unwrap() / 2.55f64 * 4.0f64)
            + (mean(&data) / 2.55f64 * 3.0f64)
        )
    ).clamp(0f64, 95f64).round() as u8
}

// TODO make this a u8
fn get_curr_brightness() -> u32 {
    let output = Command::new("brightnessctl")
        .arg("i")
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e)).unwrap();

    if output.status.success() {
        let re = Regex::new(r"\((\d+)%\)").expect("Invalid regex pattern");

        let output_str = String::from_utf8_lossy(&output.stdout);
        for caps in re.captures_iter(&output_str) {
            if let Some(percentage) = caps.get(1) {
                let percentage_value: u32 = percentage.as_str().parse().unwrap();
                return percentage_value;
            }
        }
        //let brightness: u32 = output_str.trim().parse().map_err(|e| {
        //    format!("Failed to parse brightness value: {}", e)
        //}).unwrap();

        //brightness
        panic!();
    } else {
        panic!();
    }
}

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

            if status.success() {
                // println!("Command executed successfully");
            } else {
                eprintln!("Command failed with exit code: {:?}", status.code());
            }
        },
        Err(e) => eprintln!("Error spawning command: {:?}", e)
    }
}

fn main() {

    // make two threads on for reading and one for writing
    // have the camera close and wait if another application is using it

    // TODO make the expects not crash and instead just loop again
    let handler = thread::spawn(|| {
        // (brightness value, camera value)
        let mut hist: Vec<(u32, u8)> = vec![(10u32, 150u8); 5];
        let mut initialized = false;
        let mut offset: f64 = 0f64;
        loop {
            match Device::new(0) {
                Ok(mut dev) => {
                    thread::sleep(time::Duration::from_millis(500));
                    let mut stream =
                    MmapStream::with_buffers(&mut dev, Type::VideoCapture, 4).expect("Failed to create buffer stream");
                    let (buf, _meta) = stream.next().unwrap();

                    let bri = get_curr_brightness();

                    offset += bri as f64 - hist[0].0 as f64;
                    offset *= initialized as u8 as f64;

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
                        set_brightness(&new_brightness);
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

    handler.join().unwrap();
}
