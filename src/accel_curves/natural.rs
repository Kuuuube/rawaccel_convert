use crate::types::AccelArgs;

pub fn natural(x: f64, args: &AccelArgs) -> f64 {
    if args.gain {
        return natural_gain(x, args);
    } else {
        return natural_legacy(x, args);
    }
}

fn natural_legacy(x: f64, args: &AccelArgs) -> f64 {
    //natural_base
    let offset = args.input_offset;
    let limit = args.limit - 1.0;
    let accel = args.decay_rate / f64::abs(limit);

    //operator
    if x <= offset {
        return 1.0;
    }

    let offset_x: f64 = offset - x;
    let decay: f64 = (accel * offset_x).exp();
    return limit * (1.0 - (offset - decay * offset_x) / x) + 1.0;
}

fn natural_gain(x: f64, args: &AccelArgs) -> f64 {
    //natural_base
    let offset = args.input_offset;
    let limit = args.limit - 1.0;
    let accel = args.decay_rate / f64::abs(limit);
    let constant = -limit / accel;

    //operator
    if x <= offset {
        return 1.0;
    }

    let offset_x: f64 = offset - x;
    let decay: f64 = (accel * offset_x).exp();
    let output: f64 = limit * (decay / accel - offset_x) + constant;
    return output / x + 1.0;
}
