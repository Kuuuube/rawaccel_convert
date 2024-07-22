use crate::{
    types::{AccelArgs, CapMode, Vec2},
    utility::minsd,
};

pub fn classic(x: f64, args: &AccelArgs) -> f64 {
    if args.gain {
        return classic_gain(x, args);
    } else {
        return classic_legacy(x, args);
    }
}

fn base_fn(x: f64, accel_raised: f64, args: &AccelArgs) -> f64 {
    return accel_raised * f64::powf(x - args.input_offset, args.exponent_classic) / x;
}

fn base_accel(x: f64, y: f64, args: &AccelArgs) -> f64 {
    let power = args.exponent_classic;
    return f64::powf(
        x * y * f64::powf(x - args.input_offset, -power),
        1.0 / (power - 1.0),
    );
}

fn classic_legacy(x: f64, args: &AccelArgs) -> f64 {
    let accel_raised: f64;
    let mut cap: f64 = f64::MAX;
    let mut sign: f64 = 1.0;

    //classic
    match args.cap_mode {
        CapMode::InputOutput => {
            cap = args.cap.y - 1.0;

            if cap < 0.0 {
                cap = -cap;
                sign = -sign;
            }

            {
                let exponent_classic = args.exponent_classic;
                let a: f64 = base_accel(args.cap.x, cap, args);
                accel_raised = f64::powf(a, exponent_classic - 1.0);
            }
        }
        CapMode::Input => {
            accel_raised = f64::powf(args.acceleration, args.exponent_classic - 1.0);
            if args.cap.x > 0.0 {
                let new_cap = base_fn(args.cap.x, accel_raised, args);
                cap = new_cap;
            }
        }
        CapMode::Output => {
            accel_raised = f64::powf(args.acceleration, args.exponent_classic - 1.0);

            if args.cap.y > 0.0 {
                cap = args.cap.y - 1.0;

                if cap < 0.0 {
                    cap = -cap;
                    sign = -sign;
                }
            }
        }
    }

    //operator
    if x <= args.input_offset {
        return 1.0;
    }
    return sign * minsd(base_fn(x, accel_raised, args), cap) + 1.0;
}

fn classic_gain(x: f64, args: &AccelArgs) -> f64 {
    let accel_raised: f64;
    let mut cap: Vec2 = Vec2 {
        x: f64::MAX,
        y: f64::MAX,
    };
    let mut constant: f64 = 0.0;
    let mut sign: f64 = 1.0;

    //classic
    match args.cap_mode {
        CapMode::InputOutput => {
            cap.x = args.cap.x;
            cap.y = args.cap.y - 1.0;

            if cap.y < 0.0 {
                cap.y = -cap.y;
                sign = -sign;
            }

            {
                let a: f64 = gain_accel(cap.x, cap.y, args.exponent_classic, args.input_offset);
                accel_raised = f64::powf(a, args.exponent_classic - 1.0);
            }
            constant = (base_fn(cap.x, accel_raised, args) - cap.y) * cap.x;
        }
        CapMode::Input => {
            accel_raised = f64::powf(args.acceleration, args.exponent_classic - 1.0);
            if args.cap.x > 0.0 {
                cap.x = args.cap.x;
                cap.y = gain(
                    cap.x,
                    args.acceleration,
                    args.exponent_classic,
                    args.input_offset,
                );
                constant = (base_fn(cap.x, accel_raised, args) - cap.y) * cap.x;
            }
        }
        CapMode::Output => {
            accel_raised = f64::powf(args.acceleration, args.exponent_classic - 1.0);

            if args.cap.y > 0.0 {
                cap.y = args.cap.y - 1.0;

                if cap.y == 0.0 {
                    cap.x = 0.0;
                } else {
                    if cap.y < 0.0 {
                        cap.y = -cap.y;
                        sign = -sign;
                    }

                    cap.x = gain_inverse(
                        cap.y,
                        args.acceleration,
                        args.exponent_classic,
                        args.input_offset,
                    );
                    constant = (base_fn(cap.x, accel_raised, args) - cap.y) * cap.x;
                }
            }
        }
    }

    //operator
    let output: f64;

    if x <= args.input_offset {
        return 1.0;
    }

    if x < cap.x {
        output = base_fn(x, accel_raised, args);
    } else {
        output = constant / x + cap.y;
    }

    return sign * output + 1.0;
}

fn gain(x: f64, accel: f64, power: f64, offset: f64) -> f64 {
    return power * f64::powf(accel * (x - offset), power - 1.0);
}

fn gain_inverse(y: f64, accel: f64, power: f64, offset: f64) -> f64 {
    return (accel * offset + f64::powf(y / power, 1.0 / (power - 1.0))) / accel;
}

fn gain_accel(x: f64, y: f64, power: f64, offset: f64) -> f64 {
    return -f64::powf(y / power, 1.0 / (power - 1.0)) / (offset - x);
}
