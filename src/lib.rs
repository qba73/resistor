use std::io;

pub fn get_band_input() -> u32 {
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
