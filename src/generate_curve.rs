use crate::{
    convert_curve,
    types::{AccelArgs, CurvegenResult, Point, PointScaling, Steps},
    utility::{self, lerp},
};

pub fn generate_curve(args: &AccelArgs) -> CurvegenResult {
    let curve_steps = match args.point_scaling {
        PointScaling::Libinput | PointScaling::LibinputDebug => {
            step_maker(64, 0.0, (args.dpi / 20) as f64)
        },
        _ => {
            step_maker(args.point_count * 100, 0.0, (args.dpi / 20) as f64)
        },
    };
    let mut curve_outputs: Vec<Point> = vec![];
    for curve_step in curve_steps.x_values {
        let output_sens = match args.mode {
            crate::types::AccelMode::Classic | crate::types::AccelMode::Linear => crate::classic(curve_step, args),
            crate::types::AccelMode::Jump => crate::jump(curve_step, args),
            crate::types::AccelMode::Natural => crate::natural(curve_step, args),
            crate::types::AccelMode::Synchronous => crate::synchronous(curve_step, args),
            crate::types::AccelMode::Power => crate::power(curve_step, args),
            crate::types::AccelMode::Noaccel => crate::noaccel(curve_step, args),
        };
        curve_outputs.push(Point {
            x: curve_step,
            y: output_sens * args.sens_multiplier,
        });
    }
    match args.point_scaling {
        PointScaling::Velocity | PointScaling::Libinput | PointScaling::LibinputDebug => {
            curve_outputs = convert_curve::sensitivity_to_velocity(curve_outputs)
        }
        _ => {}
    }

    return CurvegenResult {points: optimized_decimation(curve_outputs, args.point_count), step_size: curve_steps.step_size};
}

fn step_maker(step_count: u32, start: f64, end: f64) -> Steps {
    let step_distance = 1.0 / (step_count - 1) as f64;
    let mut steps: Vec<f64> = vec![];
    for i in 0..step_count {
        steps.push(utility::lerp(start, end, i as f64 * step_distance));
    }
    let step_size = utility::lerp(start, end, step_distance);
    return Steps {x_values: steps, step_size: step_size};
}

//ramer douglas peuker line decimation
fn perpendicular_distance(p: &Point, p1: &Point, p2: &Point) -> f64 {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    (p.x * dy - p.y * dx + p2.x * p1.y - p2.y * p1.x).abs() / dx.hypot(dy)
}

fn rdp(points: &[Point], epsilon: f64, result: &mut Vec<Point>) {
    let n = points.len();
    if n < 2 {
        return;
    }
    let mut max_dist = 0.0;
    let mut index = 0;
    for i in 1..n - 1 {
        let dist = perpendicular_distance(&points[i], &points[0], &points[n - 1]);
        if dist > max_dist {
            max_dist = dist;
            index = i;
        }
    }
    if max_dist > epsilon {
        rdp(&points[0..=index], epsilon, result);
        rdp(&points[index..n], epsilon, result);
    } else {
        result.push(points[n - 1]);
    }
}

fn ramer_douglas_peucker(points: Vec<Point>, epsilon: f64) -> Vec<Point> {
    let mut result = Vec::new();
    if points.len() > 0 && epsilon >= 0.0 {
        result.push(points[0]);
        rdp(&points, epsilon, &mut result);
    }
    result
}

const MIN_DECIMATION_BOUND: f64 = f64::EPSILON;
//number of iterations to halve down to 0 (worst case scenario for binary search), calculated as f64
//2099 iterations: f64::MAX
//1203 iterations: f32::MAX as f64
//1075 iterations: 1.0
const MAX_DECIMATION_BOUND: f64 = f32::MAX as f64;
const SEARCH_ITERATION_LIMIT: usize = 1203;

fn optimized_decimation(points: Vec<Point>, target_number: u32) -> Vec<Point> {
    if points.len() as u32 <= target_number {
        return points;
    }

    let mut min_bound = MIN_DECIMATION_BOUND;
    let mut max_bound = MAX_DECIMATION_BOUND;
    for i in 0..SEARCH_ITERATION_LIMIT {
        let decimation_tolerance = lerp(min_bound, max_bound, 0.5);
        let new_decimated_points = ramer_douglas_peucker(points.clone(), decimation_tolerance);
        let new_decimated_points_len = new_decimated_points.len() as u32;
        if new_decimated_points_len == target_number
            || min_bound >= max_bound
            || max_bound <= min_bound
            || i == SEARCH_ITERATION_LIMIT - 1
        {
            return new_decimated_points;
        }

        if new_decimated_points_len > target_number {
            min_bound = decimation_tolerance;
        } else {
            max_bound = decimation_tolerance;
        }
    }

    panic!("fn optimized_decimation escaped bounds");
}
