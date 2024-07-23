use core::f64;

use crate::{
    types::{AccelArgs, CapMode, Vec2},
    utility::minsd,
};

pub fn power(x: f64, args: &AccelArgs) -> f64 {
    if args.gain {
        return power_gain(x, args);
    } else {
        return power_legacy(x, args);
    }
}

fn base_fn(x: f64, args: &AccelArgs, offset: Vec2, scale: f64, constant: f64) -> f64 {
    if x <= offset.x {
        return offset.y;
    } else {
        return f64::powf(scale * x, args.exponent_power) + constant / x;
    }
}

fn power_legacy(x: f64, args: &AccelArgs) -> f64 {
    //power_base
    let mut offset: Vec2;
    let scale: f64;
    let mut constant: f64;

    let n: f64 = args.exponent_power;

    if !matches!(args.cap_mode, CapMode::InputOutput) {
        scale = args.scale;
        offset = Vec2 {
            x: gain_inverse(args.output_offset, n, scale),
            y: args.output_offset,
        };
        constant = offset.x * offset.y * n / (n + 1.0);
    } else if args.gain {
        scale = scale_from_gain_point(args.cap.x, args.cap.y, n);
        offset = Vec2 {
            x: gain_inverse(args.output_offset, n, scale),
            y: args.output_offset,
        };
        constant = offset.x * offset.y * n / (n + 1.0);
    } else {
        offset = Default::default();
        constant = 0.0;
        scale = scale_from_output_point(args.cap.x, args.cap.y, n, constant);
    }

    //power
    let mut cap: f64 = f64::MAX;

    match args.cap_mode {
        CapMode::InputOutput => {
            cap = args.cap.y;
        }
        CapMode::Input => {
            if args.cap.x > 0.0 {
                cap = base_fn(args.cap.x, args, offset, scale, constant);
            }
        }
        CapMode::Output => {
            if args.cap.y > 0.0 {
                cap = args.cap.y;
            }
        }
    }

    //operator
    return minsd(base_fn(x, args, offset, scale, constant), cap);
}

fn power_gain(x: f64, args: &AccelArgs) -> f64 {
    //power_base
    let mut offset: Vec2;
    let scale: f64;
    let mut constant: f64;

    let n: f64 = args.exponent_power;

    if !matches!(args.cap_mode, CapMode::InputOutput) {
        scale = args.scale;
        offset = Vec2 {
            x: gain_inverse(args.output_offset, n, scale),
            y: args.output_offset,
        };
        constant = offset.x * offset.y * n / (n + 1.0);
    } else if args.gain {
        scale = scale_from_gain_point(args.cap.x, args.cap.y, n);
        offset = Vec2 {
            x: gain_inverse(args.output_offset, n, scale),
            y: args.output_offset,
        };
        constant = offset.x * offset.y * n / (n + 1.0);
    } else {
        offset = Default::default();
        constant = 0.0;
        scale = scale_from_output_point(args.cap.x, args.cap.y, n, constant);
    }

    //power
    let mut cap: Vec2 = Vec2 {
        x: f64::MAX,
        y: f64::MAX,
    };
    let constant_b: f64;

    match args.cap_mode {
        CapMode::InputOutput => {
            cap = args.cap;
        }
        CapMode::Input => {
            if args.cap.x > 0.0 {
                cap.x = args.cap.x;
                cap.y = gain(args.cap.x, args.exponent_power, scale);
            }
        }
        CapMode::Output => {
            if args.cap.y > 0.0 {
                cap.x = gain_inverse(args.cap.y, args.exponent_power, scale);
                cap.y = args.cap.y;
            }
        }
    }

    if args.cap.x <= offset.x && matches!(args.cap_mode, CapMode::Input) {
        cap.x = 0.0;
        cap.y = offset.y;
        constant_b = 0.0;
    } else {
        constant_b =
            integration_constant(cap.x, cap.y, base_fn(cap.x, args, offset, scale, constant));
    }

    //operator
    if x < cap.x {
        return base_fn(x, args, offset, scale, constant);
    } else {
        return cap.y + constant_b / x;
    }
}

fn integration_constant(input: f64, gain: f64, output: f64) -> f64 {
    return (output - gain) * input;
}

fn gain(input: f64, power: f64, scale: f64) -> f64 {
    return (power + 1.0) * f64::powf(input * scale, power);
}

fn gain_inverse(gain: f64, power: f64, scale: f64) -> f64 {
    return f64::powf(gain / (power + 1.0), 1.0 / power) / scale;
}

fn scale_from_gain_point(input: f64, gain: f64, power: f64) -> f64 {
    return f64::powf(gain / (power + 1.0), 1.0 / power) / input;
}

fn scale_from_output_point(input: f64, output: f64, power: f64, c: f64) -> f64 {
    return f64::powf(output - c / input, 1.0 / power) / input;
}
