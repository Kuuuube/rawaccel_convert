use crate::types::Point;

pub fn sensitivity_to_velocity(sensitivity_accel_curve: Vec<Point>) -> Vec<Point> {
    let mut new_accel_curve: Vec<Point> = vec![];
    for sensitivity_point in sensitivity_accel_curve {
        new_accel_curve.push(sensitivity_point_to_velocity(sensitivity_point))
    }
    return new_accel_curve;
}

pub fn sensitivity_point_to_velocity(sensitivity_point: Point) -> Point {
    return Point {
        x: sensitivity_point.x,
        y: sensitivity_point.x * sensitivity_point.y,
    };
}
