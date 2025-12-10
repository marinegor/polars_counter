use std::fs::File;
mod expressions;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let data: expressions::Counter = serde_pickle::from_reader(
        File::open(file_path).unwrap(),
        serde_pickle::de::DeOptions::default(),
    )
    .unwrap();
    println!("{:?}", data);
}
