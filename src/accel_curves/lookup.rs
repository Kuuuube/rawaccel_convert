use crate::{
    types::{AccelArgs, Vec2}, unwrap_option_or_return_none, utility::{maxsd, minsd, LUT_POINTS_CAPACITY}
};

pub fn lookup(x: f64, args: &AccelArgs) -> Option<f64> {
    let capacity = LUT_POINTS_CAPACITY;

    let size: i32 = (args.lookup_data.len() / 2) as i32;
    let velocity: bool = args.gain;

    //operator
    let points = &args.lookup_data;

    let mut lo: i32 = 0;
    let mut hi: i32 = size - 2;

    if x <= 0.0 {
        return Some(0.0);
    }

    if (hi as i64) < ((capacity - 1) as i64) {
        while lo <= hi {
            let mid: i32 = (lo + hi) / 2;
            let p: Vec2 = unwrap_option_or_return_none!(points.get(mid as usize)).to_owned();

            if x < p.x {
                hi = mid - 1;
            } else if x > p.x {
                lo = mid + 1;
            } else {
                let mut y: f64 = p.y;
                if velocity {
                    y /= x;
                }
                return Some(y);
            }
        }

        if lo > 0 {
            let a: Vec2 = unwrap_option_or_return_none!(points.get((lo - 1) as usize)).to_owned();
            let b: Vec2 = unwrap_option_or_return_none!(points.get(lo as usize)).to_owned();
            let t: f64 = (x - a.x) / (b.x - a.x);
            let mut y: f64 = lerp(a.y, b.y, t);
            if velocity {
                y /= x;
            }
            return Some(y);
        }
    }

    let mut y: f64 = unwrap_option_or_return_none!(points.get(0)).y;
    if velocity {
        y /= unwrap_option_or_return_none!(points.get(0)).x;
    }
    return Some(y);
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    let x: f64 = a + t * (b - a);
    if (t > 1.0) == (a < b) {
        return maxsd(x, b);
    }
    return minsd(x, b);
}
