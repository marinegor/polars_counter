use std::env;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("{:?}", file_path);
    println!("------------------------------------------------------------");

    let data: serde_pickle::Value = serde_pickle::from_reader(
        File::open(file_path).unwrap(),
        serde_pickle::de::DeOptions::default(),
    )
    .expect("Could not unwrap data");
    println!("{:?}", data);
}
