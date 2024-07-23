use std::env;

use accel_curves::classic::classic;
use accel_curves::jump::jump;
use accel_curves::natural::natural;
use accel_curves::noaccel::noaccel;
use accel_curves::power::power;
use accel_curves::synchronous::synchronous;
use types::{AccelArgs, CapMode};

mod accel_curves;
mod convert_curve;
mod generate_curve;
mod types;
mod utility;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut accel_args = AccelArgs::default();
    accel_args.sens_multiplier = 1.0;
    accel_args.gain = true;
    accel_args.cap_mode = CapMode::Output;

    let accel_curve = match args.get(1) {
        Some(some) => some,
        None => {
            println!("Invalid args");
            return;
        }
    };

    accel_args.mode = match accel_curve.as_str() {
        "linear" | "classic" => types::AccelMode::Classic,
        "jump" => types::AccelMode::Jump,
        "natural" => types::AccelMode::Natural,
        "synchronous" => types::AccelMode::Synchronous,
        "power" => types::AccelMode::Power,
        _ => types::AccelMode::Noaccel,
    };
    let libinput_curve =
        convert_curve::sensitivity_to_velocity(generate_curve::generate_curve(&accel_args, 64));
    println!("{:?}", libinput_curve);
}
