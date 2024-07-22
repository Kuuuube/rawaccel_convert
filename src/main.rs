use types::AccelArgs;

mod classic;
mod types;
mod utility;

fn main() {
    //let mut args = AccelArgs::default();
    //args.acceleration = 0.0315;
    //args.exponent_classic = 2.0;
    //args.cap_mode = types::CapMode::input;
    //args.cap = [34.0, 0.00001];

    // let sens_multiplier = 1.0;
    // let accel = classic::classic(80.0, &args) * sens_multiplier;
    // println!("{}", accel);

    make_points();
}

fn make_points() {
    let mut args = AccelArgs::default();
    let sens_multiplier = 1.0;

    for i in 0..80 {
        let accel = classic::classic(i as f64, &args) * sens_multiplier;
        print!("({},{}),", i, accel);
    }
    print!("\n");
}