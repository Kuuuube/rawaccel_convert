use crate::types::AccelArgs;

pub fn noaccel(_x: f64, _args: &AccelArgs) -> f64 {
    return 1.0;
}
