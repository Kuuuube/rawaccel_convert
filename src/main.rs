use std::env;

use accel_curves::classic::classic;
use accel_curves::jump::jump;
use accel_curves::natural::natural;
use accel_curves::noaccel::noaccel;
use accel_curves::power::power;
use accel_curves::synchronous::synchronous;

mod accel_curves;
mod args_parser;
mod convert_curve;
mod generate_curve;
mod macros;
mod types;
mod utility;

fn main() {
    let args: Vec<String> = env::args().collect();
    let accel_args = args_parser::parser(args).expect("Args parsing failed");

    let curve = generate_curve::generate_curve(&accel_args);
    println!("{:?}", curve);
}
