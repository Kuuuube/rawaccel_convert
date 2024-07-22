use crate::types::{AccelArgs, Vec2};

const SMOOTH_SCALE: f64 = 2.0 * 3.14159265358979323846;

pub fn jump(x: f64, args: &AccelArgs) -> f64 {
    if args.gain {
        return jump_gain(x, args);
    } else {
        return jump_legacy(x, args);
    }
}

fn jump_legacy(x: f64, args: &AccelArgs) -> f64 {
    let smooth_rate: f64;

    //jump_base
    let step = Vec2 {
        x: args.cap.x,
        y: args.cap.y - 1.0,
    };
    {
        let rate_inverse: f64 = args.smooth * step.x;

        if rate_inverse < 1.0 {
            smooth_rate = 0.0;
        } else {
            smooth_rate = SMOOTH_SCALE / rate_inverse;
        }
    }

    //operator
    if is_smooth(smooth_rate) {
        return smooth(x, smooth_rate, step) + 1.0;
    } else if x < step.x {
        return 1.0;
    } else {
        return 1.0 + step.y;
    }
}

fn jump_gain(x: f64, args: &AccelArgs) -> f64 {
    let smooth_rate: f64;

    //jump_base
    let step = Vec2 {
        x: args.cap.x,
        y: args.cap.y - 1.0,
    };
    {
        let rate_inverse: f64 = args.smooth * step.x;

        if rate_inverse < 1.0 {
            smooth_rate = 0.0;
        } else {
            smooth_rate = SMOOTH_SCALE / rate_inverse;
        }
    }

    //jump
    let c: f64;
    c = -smooth_antideriv(0.0, smooth_rate, step);

    //operator
    if x <= 0.0 {
        return 1.0;
    }

    if is_smooth(smooth_rate) {
        return 1.0 + (smooth_antideriv(x, smooth_rate, step) + c) / x;
    }

    if x < step.x {
        return 1.0;
    } else {
        return 1.0 + step.y * (x - step.x) / x;
    }
}

fn is_smooth(smooth_rate: f64) -> bool {
    return smooth_rate != 0.0;
}

fn decay(x: f64, smooth_rate: f64, step: Vec2) -> f64 {
    return (smooth_rate * (step.x - x)).exp();
}

fn smooth(x: f64, smooth_rate: f64, step: Vec2) -> f64 {
    return step.y / (1.0 + decay(x, smooth_rate, step));
}

fn smooth_antideriv(x: f64, smooth_rate: f64, step: Vec2) -> f64 {
    return step.y * (x + (1.0 + decay(x, smooth_rate, step)).ln() / smooth_rate);
}
