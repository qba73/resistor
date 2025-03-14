use std::{io, process::exit};

fn main() {
    println!("Please enter band colour:");
    println!(
        "Available colours: black, brown, red, orange, yellow, green, blue, violet, grey, white"
    );

    let band1 = get_band_input();
    let band2 = get_band_input();
    let band3 = get_band_input();

    let mut r1:usize = (band1 * 10) as usize;
    r1 = r1 + band2 as usize;
    

    let Some(magnitude) = 10_u32.checked_pow(band3) else {
        println!("error");
        exit(1);   
    };


    r1 = r1.checked_mul(magnitude as usize).expect("overflow");

    let resistor = Component::Resistor(r1);
    println!("{:?}", resistor);


}

fn get_band_input() -> u32 {
    let mut color = String::new();

    io::stdin()
        .read_line(&mut color)
        .expect("Failed to read the line");

    let val = match color.trim() {
        "black" => 0,
        "brown" => 1,
        "red" => 2,
        "orange" => 3,
        "yellow" => 4,
        "green" => 5,
        "blue" => 6,
        "violet" => 7,
        "grey" => 8,
        "white" => 9,
        _ => unreachable!("PANIC!"),
    };
    val
}


#[derive(Debug)]
pub enum Component {
    Diode,
    Resistor(usize),
    Capacitor(usize),
}