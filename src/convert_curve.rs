use crate::types::Point;

pub fn sensitivity_to_velocity(sensitivity_accel_curve: Vec<Point>) -> Vec<Point> {
    let mut new_accel_curve: Vec<Point> = vec![];
    for sensitivity_point in sensitivity_accel_curve {
        new_accel_curve.push(Point {
            x: sensitivity_point.x,
            y: sensitivity_point.y * sensitivity_point.x,
        })
    }
    return new_accel_curve;
}
