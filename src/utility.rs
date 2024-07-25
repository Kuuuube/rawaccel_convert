pub const LUT_RAW_DATA_CAPACITY: u32 = 514;
pub const LUT_POINTS_CAPACITY: u32 = LUT_RAW_DATA_CAPACITY / 2;

pub fn minsd(a: f64, b: f64) -> f64 {
    if a < b {
        return a;
    } else {
        return b;
    }
}


pub fn maxsd(a: f64, b: f64) -> f64 {
    if b < a {
        return a;
    } else {
        return b;
    }
}

//taken from libm
//https://github.com/rust-lang/libm/blob/279e5f6abe0a2ca9066962d9ec894f0df1f417ac/src/math/mod.rs#L1-L5
//https://github.com/rust-lang/libm/blob/279e5f6abe0a2ca9066962d9ec894f0df1f417ac/src/math/ilogb.rs
macro_rules! force_eval {
    ($e:expr) => {
        unsafe { ::core::ptr::read_volatile(&$e) }
    };
}

const FP_ILOGBNAN: i32 = -1 - 0x7fffffff;
const FP_ILOGB0: i32 = FP_ILOGBNAN;

pub fn ilogb(x: f64) -> i32 {
    let mut i: u64 = x.to_bits();
    let e = ((i >> 52) & 0x7ff) as i32;

    if e == 0 {
        i <<= 12;
        if i == 0 {
            force_eval!(0.0 / 0.0);
            return FP_ILOGB0;
        }
        /* subnormal x */
        let mut e = -0x3ff;
        while (i >> 63) == 0 {
            e -= 1;
            i <<= 1;
        }
        e
    } else if e == 0x7ff {
        force_eval!(0.0 / 0.0);
        if (i << 12) != 0 {
            FP_ILOGBNAN
        } else {
            i32::max_value()
        }
    } else {
        e - 0x3ff
    }
}

//taken from libm
//https://github.com/rust-lang/libm/blob/a0a5bd85c913fe5f6c42bc11ef292e575f97ca6d/src/math/scalbn.rs
pub fn scalbn(x: f64, mut n: i32) -> f64 {
    let x1p1023 = f64::from_bits(0x7fe0000000000000); // 0x1p1023 === 2 ^ 1023
    let x1p53 = f64::from_bits(0x4340000000000000); // 0x1p53 === 2 ^ 53
    let x1p_1022 = f64::from_bits(0x0010000000000000); // 0x1p-1022 === 2 ^ (-1022)

    let mut y = x;

    if n > 1023 {
        y *= x1p1023;
        n -= 1023;
        if n > 1023 {
            y *= x1p1023;
            n -= 1023;
            if n > 1023 {
                n = 1023;
            }
        }
    } else if n < -1022 {
        /* make sure final n < -53 to avoid double
        rounding in the subnormal range */
        y *= x1p_1022 * x1p53;
        n += 1022 - 53;
        if n < -1022 {
            y *= x1p_1022 * x1p53;
            n += 1022 - 53;
            if n < -1022 {
                n = -1022;
            }
        }
    }
    y * f64::from_bits(((0x3ff + n) as u64) << 52)
}

pub fn lerp(start: f64, end: f64, input: f64) -> f64 {
    start + input * (end - start)
}
