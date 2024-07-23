use std::cmp::min;

use crate::{
    types::{AccelArgs, FpRepRange},
    utility::{ilogb, lerp, scalbn, LUT_RAW_DATA_CAPACITY},
};

pub fn synchronous(x: f64, args: &AccelArgs) -> f64 {
    if args.gain {
        return synchronous_gain(x, args);
    } else {
        return synchronous_legacy(x, args);
    }
}

fn synchronous_legacy(x: f64, args: &AccelArgs) -> f64 {
    let log_motivity: f64 = args.motivity.ln();
    let gamma_const: f64 = args.gamma / log_motivity;
    let log_syncspeed: f64 = args.sync_speed.ln();
    let syncspeed: f64 = args.sync_speed;
    let sharpness: f64 = if args.smooth == 0.0 {
        16.0
    } else {
        0.5 / args.smooth
    };
    let sharpness_recip: f64 = 1.0 / sharpness;
    let use_linear_clamp: bool = sharpness >= 16.0;
    let minimum_sens: f64 = 1.0 / args.motivity;
    let maximum_sens: f64 = args.motivity;

    //operator
    if use_linear_clamp {
        let log_space: f64 = gamma_const * (x.ln() - log_syncspeed);

        if log_space < -1.0 {
            return minimum_sens;
        }

        if log_space > 1.0 {
            return maximum_sens;
        }

        return (log_space * log_motivity).exp();
    }

    if x == syncspeed {
        return 1.0;
    }

    let log_x: f64 = x.ln();
    let log_diff: f64 = log_x - log_syncspeed;

    if log_diff > 0.0 {
        let log_space: f64 = gamma_const * log_diff;
        let exponent: f64 = f64::powf(f64::powf(log_space, sharpness).tanh(), sharpness_recip);
        return (exponent * log_motivity).exp();
    } else {
        let log_space: f64 = -gamma_const * log_diff;
        let exponent: f64 = -f64::powf(f64::powf(log_space, sharpness).tanh(), sharpness_recip);
        return (exponent * log_motivity).exp();
    }
}

fn synchronous_gain(x: f64, args: &AccelArgs) -> f64 {
    let capacity = LUT_RAW_DATA_CAPACITY;
    let velocity: bool = true;
    let range = FpRepRange { start: -3, stop: 9, num: 8 };
    let x_start: f64 = scalbn(1.0, range.start);

    let mut sum: f64 = 0.0;
    let mut a: f64 = 0.0;

    let mut args_data: Vec<f64> = vec![];

    let mut e: i32 = 0;
    while e < range.stop - range.start {
        let exp_scale: f64 = scalbn(1.0, e + range.start) / range.num as f64;
        let mut i: i32 = 0;
        while i < range.num {
            args_data.push(make_args_data((i + range.num) as f64 * exp_scale, &mut sum, &mut a, velocity, &args));
            i += 1;
        }
        e += 1;
    }

    args_data.push(make_args_data(scalbn(1.0, range.stop), &mut sum, &mut a, velocity, &args));

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
                data[idx as usize],
                data[(idx + 1) as usize],
                idx_f - idx as f64,
            );
            if velocity {
                y /= x;
            }
            return y;
        }
    }

    let mut y: f64 = data[0];
    if velocity {
        y /= x_start;
    }
    return y;
}

fn make_args_data(x: f64, sum: &mut f64, a: &mut f64, velocity: bool, args: &AccelArgs) -> f64 {
    let b: f64 = x;
    let partitions: i32 = 2;

    let interval: f64 = (b - *a) / partitions as f64;
    let mut i = 1;
    while i <= partitions {
        *sum += synchronous_legacy(*a + i as f64 * interval, args) * interval;
        i += 1;
    }
    *a = b;
    let mut y: f64 = *sum;
    if !velocity {
        y /= x;
    }
    return y;
}
