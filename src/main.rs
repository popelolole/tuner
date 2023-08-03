

const REFERENCE_PITCH : f64 = 440.0;
const ALL_NOTES : [&str; 12] = 
    ["A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#"];

fn find_closest_note(pitch: f64) -> String {
    let i : i32 = ((pitch / REFERENCE_PITCH).log2() * 12.0).round() as i32;
    let closest_note = ALL_NOTES[(((i%12)+12)%12) as usize].to_owned() + 
        &(4.0 + ((i as f64 + 9.0)/12.0).floor()).to_string();
    return closest_note;
}

fn main() {
    println!("{REFERENCE_PITCH}");
    println!("{:?}", ALL_NOTES);
    println!("{}", find_closest_note(450.0));
}
