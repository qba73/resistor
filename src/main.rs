use std::io;

fn main() {
    println!("Please enter band colour:");

    let band1 = get_band_input();
    let band2 = get_band_input();
    let band3 = get_band_input();

    let r1 = ((band1 * 10) + band2) * 10_u32.pow(band3);
    println!("{r1}");
}

fn get_band_input() -> u32 {
    let mut color = String::new();

    io::stdin()
        .read_line(&mut color)
        .expect("Failed to read the line");

    let val = match color.trim() {
        "red" => 2,
        "brown" => 1,
        "black" => 0,
        _ => unreachable!("PANIC!"),
    };
    val
}
