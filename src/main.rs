use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use std::time::Duration;

const REFERENCE_PITCH : f64 = 440.0;
const ALL_NOTES : [&str; 12] = 
    ["A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#"];

fn find_closest_note(pitch: f64) -> String {
    let i : i32 = ((pitch / REFERENCE_PITCH).log2() * 12.0).round() as i32;
    let closest_note = ALL_NOTES[(((i%12)+12)%12) as usize].to_string() + 
        &(4.0 + ((i as f64 + 9.0)/12.0).floor()).to_string();
    return closest_note;
}

fn main() {
    println!("{REFERENCE_PITCH}");
    println!("{:?}", ALL_NOTES);
    println!("{}", find_closest_note(450.0));

    let host = cpal::default_host();
    let device = host.default_input_device().expect("no input device available");
    let mut supported_configs_range = device.supported_input_configs()
        .expect("error while querying configs");
    let supported_config = supported_configs_range.next()
        .expect("no supported config?!")
        .with_max_sample_rate();
    let config = supported_config.into();
    let recorded_data: Arc<Mutex<Vec<f32>>> = Arc::new(Mutex::new(Vec::new()));
    let cloned_data = Arc::clone(&recorded_data);

    let stream = device.build_input_stream(
        &config,
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let mut recorded_data = cloned_data.lock().unwrap();
            recorded_data.extend_from_slice(data);
            //println!("{:#?}", data);
            // react to stream events and read or write stream data here.
        },
        move |err| {
            eprintln!("Error: {:?}", err);      
        },
        None // None=blocking, Some(Duration)=timeout
    ).expect("Failed to build input stream");

    stream.play().expect("Failed to play stream");

    // Sleep for a few seconds to allow recording
    std::thread::sleep(Duration::from_secs(5));

    // Stop the audio stream
    stream.pause().expect("Failed to pause stream");

    // Access the recorded audio data
    let recorded_data = recorded_data.lock().unwrap();

    println!("Recorded {} samples", recorded_data.len());
}
