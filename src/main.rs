use resistor::get_band_input;
use resistor::Component;
use std::process::exit;

fn main() {
    println!("Please enter band colour:");
    println!(
        "Available colours: black, brown, red, orange, yellow, green, blue, violet, grey, white"
    );

    let band1 = get_band_input();
    let band2 = get_band_input();
    let band3 = get_band_input();

    let mut r1: usize = (band1 * 10) as usize;
    r1 = r1 + band2 as usize;

    let Some(magnitude) = 10_u32.checked_pow(band3) else {
        println!("error");
        exit(1);
    };

    r1 = r1.checked_mul(magnitude as usize).expect("overflow");

    let resistor = Component::Resistor(r1);
    println!("{:?}", resistor);
}
