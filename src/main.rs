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
    let mut args = AccelArgs::default();
    let sens_multiplier = 1.0;
    args.gain = true;
    args.cap_mode = CapMode::Output;

    for i in 0..80 {
        let x = i as f64;
        let noaccel_curve = noaccel(x, &args) * sens_multiplier;
        let classic_curve = classic(x, &args) * sens_multiplier;
        let jump_curve = jump(x, &args) * sens_multiplier;
        let natural_curve = natural(x, &args) * sens_multiplier;
        let synchronous_curve = synchronous(x, &args) * sens_multiplier;
        let power_curve = power(x, &args) * sens_multiplier;
        println!(
            "{}: {} {} {} {} {} {}",
            x,
            noaccel_curve,
            classic_curve,
            jump_curve,
            natural_curve,
            synchronous_curve,
            power_curve
        );
    }
    print!("\n");
}
