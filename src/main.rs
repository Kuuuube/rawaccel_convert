use std::env;

use accel_curves::classic::classic;
use accel_curves::jump::jump;
use accel_curves::natural::natural;
use accel_curves::noaccel::noaccel;
use accel_curves::power::power;
use accel_curves::synchronous::synchronous;
use types::{AccelArgs, CapMode};

mod accel_curves;
mod types;
mod utility;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut accel_args = AccelArgs::default();
    let sens_multiplier = 1.0;
    accel_args.gain = true;
    accel_args.cap_mode = CapMode::Output;

    let accel_curve = match args.get(1) {
        Some(some) => some,
        None => {
            println!("Invalid args");
            return;
        }
    };

    for i in 0..80 {
        let x = i as f64;
        let accel = sens_multiplier
            * match accel_curve.as_str() {
                "linear" | "classic" => classic(x, &accel_args),
                "jump" => jump(x, &accel_args),
                "natural" => natural(x, &accel_args),
                "synchronous" => synchronous(x, &accel_args),
                "power" => power(x, &accel_args),
                _ => noaccel(x, &accel_args),
            };
        print!("({},{}),", x, accel);
    }
    print!("\n");
}
