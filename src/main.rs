use std::env;
use std::fs::File;

fn with_value() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("{:?}", file_path);
    println!("------------------------------------------------------------");

    type Active = serde_pickle::Value;

    let data: Active = serde_pickle::from_reader(
        File::open(file_path).unwrap(),
        serde_pickle::de::DeOptions::default(),
    )
    .expect("Could not unwrap data");
    println!("{:?}", data);
}

fn with_bytes() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("{:?}", file_path);
    println!("------------------------------------------------------------");

    type Active = Vec<u8>;

    let data: Active = serde_pickle::from_reader(
        File::open(file_path).unwrap(),
        serde_pickle::de::DeOptions::default(),
    )
    .expect("Could not unwrap data");
    println!("{:?}", data);
}

fn main() {
    with_value();
    with_bytes();
}
