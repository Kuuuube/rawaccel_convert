use std::cmp::min;

use crate::{
    types::{AccelArgs, FpRepRange},
    unwrap_option_or_return_none,
    utility::{ilogb, lerp, scalbn, LUT_RAW_DATA_CAPACITY},
};

pub fn motivity(x: f64, args: &AccelArgs) -> Option<f64> {
    if args.gain {
        return motivity_gain(x, args);
    } else {
        return motivity_legacy(x, args);
    }
}

fn motivity_legacy(x: f64, args: &AccelArgs) -> Option<f64> {
    let accel: f64 = args.gamma.exp(); //args.growth_rate.exp()
    let motivity: f64 = 2.0 * args.motivity.ln();
    let midpoint: f64 = args.sync_speed.ln(); //args.midpoint.ln()
    let constant: f64 = -motivity / 2.0;

    //operator
    let denom: f64 = (accel * (midpoint - x.ln())).exp() + 1.0;
    return Some((motivity / denom + constant).exp());
}

fn motivity_gain(x: f64, args: &AccelArgs) -> Option<f64> {
    let capacity = LUT_RAW_DATA_CAPACITY;
    let velocity: bool = true;
    let range = FpRepRange {
        start: -3,
        stop: 9,
        num: 8,
    };
    let x_start: f64 = scalbn(1.0, range.start);

    let mut sum: f64 = 0.0;
    let mut a: f64 = 0.0;

    let mut args_data: Vec<f64> = vec![];

    let mut e: i32 = 0;
    while e < range.stop - range.start {
        let exp_scale: f64 = scalbn(1.0, e + range.start) / range.num as f64;
        let mut i: i32 = 0;
        while i < range.num {
            args_data.push(unwrap_option_or_return_none!(make_args_data(
                (i + range.num) as f64 * exp_scale,
                &mut sum,
                &mut a,
                velocity,
                &args,
            )));
            i += 1;
        }
        e += 1;
    }

    args_data.push(unwrap_option_or_return_none!(make_args_data(
        scalbn(1.0, range.stop),
        &mut sum,
        &mut a,
        velocity,
        &args,
    )));

    //operator
    let data = args_data;

    let e: i32 = min(ilogb(x), range.stop - 1);

    if e >= range.start {
        let idx_int_log_part: i32 = e - range.start;
        let idx_frac_lin_part: f64 = scalbn(x, -e) - 1.0;
        let idx_f: f64 = range.num as f64 * (idx_int_log_part as f64 + idx_frac_lin_part);

        let idx: u32 = min(idx_f as i32, range.size() - 2) as u32;

        if idx < capacity - 1 {
            let mut y: f64 = lerp(
                unwrap_option_or_return_none!(data.get(idx as usize)).to_owned(),
                unwrap_option_or_return_none!(data.get((idx + 1) as usize)).to_owned(),
                idx_f - idx as f64,
            );
            if velocity {
                y /= x;
            }
            return Some(y);
        }
    }

    let mut y: f64 = unwrap_option_or_return_none!(data.get(0)).to_owned();
    if velocity {
        y /= x_start;
    }
    return Some(y);
}

fn make_args_data(
    x: f64,
    sum: &mut f64,
    a: &mut f64,
    velocity: bool,
    args: &AccelArgs,
) -> Option<f64> {
    let b: f64 = x;
    let partitions: i32 = 2;

    let interval: f64 = (b - *a) / partitions as f64;
    let mut i = 1;
    while i <= partitions {
        *sum += unwrap_option_or_return_none!(motivity_legacy(*a + i as f64 * interval, args))
            * interval;
        i += 1;
    }
    *a = b;
    let mut y: f64 = *sum;
    if !velocity {
        y /= x;
    }
    return Some(y);
}
