use std::cmp::min;

use crate::{
    types::{AccelArgs, FpRepRange},
    utility::{ilogb, lerp, scalbn, LUT_RAW_DATA_CAPACITY},
};

pub fn synchronous(x: f64, args: &AccelArgs) -> f64 {
    return synchronous_legacy(x, args);
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
