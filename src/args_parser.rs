use crate::{
    types::{self, AccelArgs, CapMode, PointScaling, Vec2},
    unwrap_option_or_continue, unwrap_result_or_continue,
};

pub fn parser(args: Vec<String>) -> Option<AccelArgs> {
    let mut accel_args = AccelArgs::default();

    if args.len() < 2 {
        print_help_and_exit();
    }

    accel_args.mode = match args[1].as_str() {
        "linear" => types::AccelMode::Linear,
        "classic" => types::AccelMode::Classic,
        "jump" => types::AccelMode::Jump,
        "natural" => types::AccelMode::Natural,
        "synchronous" => types::AccelMode::Synchronous,
        "power" => types::AccelMode::Power,
        "motivity" => types::AccelMode::Motivity,
        "lookup" => types::AccelMode::Lookup,
        _ => types::AccelMode::Noaccel,
    };

    //global args
    for arg in &args {
        if ["--help", "-h", "--h", "-help", "help"].contains(&arg.as_str()) {
            print_help_and_exit();
        }
        let split: (&str, &str) = unwrap_option_or_continue!(arg.split_once("="));
        match &split.0.to_lowercase() as &str {
            "--pointscaling" => {
                accel_args.point_scaling = split
                    .1
                    .parse::<PointScaling>()
                    .unwrap_or_else(|_| AccelArgs::default().point_scaling)
            }
            "--pointcount" => {
                accel_args.point_count = split
                    .1
                    .parse::<u32>()
                    .unwrap_or_else(|_| AccelArgs::default().point_count)
            }
            "--dpi" => {
                accel_args.dpi = split
                    .1
                    .parse::<u32>()
                    .unwrap_or_else(|_| AccelArgs::default().dpi)
            }
            _ => {}
        }
    }

    //curve args
    match accel_args.mode {
        types::AccelMode::Classic | types::AccelMode::Linear => {
            for arg in &args {
                let split: (&str, &str) = unwrap_option_or_continue!(arg.split_once("="));
                match &split.0.to_lowercase() as &str {
                    "--sens" => {
                        accel_args.sens_multiplier = split
                            .1
                            .parse::<f64>()
                            .unwrap_or_else(|_| AccelArgs::default().sens_multiplier)
                    }
                    "--gain" => {
                        accel_args.gain = split
                            .1
                            .parse::<bool>()
                            .unwrap_or_else(|_| AccelArgs::default().gain)
                    }
                    "--accel" => {
                        accel_args.acceleration = split
                            .1
                            .parse::<f64>()
                            .unwrap_or_else(|_| AccelArgs::default().acceleration)
                    }
                    "--captype" => {
                        accel_args.cap_mode = split
                            .1
                            .parse::<CapMode>()
                            .unwrap_or_else(|_| AccelArgs::default().cap_mode)
                    }
                    "--capin" => {
                        accel_args.cap.x = split
                            .1
                            .parse::<f64>()
                            .unwrap_or_else(|_| AccelArgs::default().cap.x)
                    }
                    "--capout" => {
                        accel_args.cap.y = split
                            .1
                            .parse::<f64>()
                            .unwrap_or_else(|_| AccelArgs::default().cap.y)
                    }
                    "--offsetin" => {
                        accel_args.input_offset = split
                            .1
                            .parse::<f64>()
                            .unwrap_or_else(|_| AccelArgs::default().input_offset)
                    }
                    "--power" => {
                        accel_args.exponent_classic = split
                            .1
                            .parse::<f64>()
                            .unwrap_or_else(|_| AccelArgs::default().exponent_classic)
                    }
                    _ => {}
                }
            }
        }
        types::AccelMode::Jump => {
            for arg in args {
                let split: (&str, &str) = unwrap_option_or_continue!(arg.split_once("="));
                match &split.0.to_lowercase() as &str {
                    "--sens" => {
                        accel_args.sens_multiplier = split
                            .1
                            .parse::<f64>()
                            .unwrap_or_else(|_| AccelArgs::default().sens_multiplier)
                    }
                    "--gain" => {
                        accel_args.gain = split
                            .1
                            .parse::<bool>()
                            .unwrap_or_else(|_| AccelArgs::default().gain)
                    }
                    "--smooth" => {
                        accel_args.smooth = split
                            .1
                            .parse::<f64>()
                            .unwrap_or_else(|_| AccelArgs::default().smooth)
                    }
                    "--input" => {
                        accel_args.cap.x = split
                            .1
                            .parse::<f64>()
                            .unwrap_or_else(|_| AccelArgs::default().cap.x)
                    }
                    "--output" => {
                        accel_args.cap.y = split
                            .1
                            .parse::<f64>()
                            .unwrap_or_else(|_| AccelArgs::default().cap.y)
                    }
                    _ => {}
                }
            }
        }
        types::AccelMode::Natural => {
            for arg in args {
                let split: (&str, &str) = unwrap_option_or_continue!(arg.split_once("="));
                match &split.0.to_lowercase() as &str {
                    "--sens" => {
                        accel_args.sens_multiplier = split
                            .1
                            .parse::<f64>()
                            .unwrap_or_else(|_| AccelArgs::default().sens_multiplier)
                    }
                    "--gain" => {
                        accel_args.gain = split
                            .1
                            .parse::<bool>()
                            .unwrap_or_else(|_| AccelArgs::default().gain)
                    }
                    "--decay" => {
                        accel_args.decay_rate = split
                            .1
                            .parse::<f64>()
                            .unwrap_or_else(|_| AccelArgs::default().decay_rate)
                    }
                    "--offsetin" => {
                        accel_args.cap.x = split
                            .1
                            .parse::<f64>()
                            .unwrap_or_else(|_| AccelArgs::default().cap.x)
                    }
                    "--limit" => {
                        accel_args.limit = split
                            .1
                            .parse::<f64>()
                            .unwrap_or_else(|_| AccelArgs::default().limit)
                    }

                    _ => {}
                }
            }
        }
        types::AccelMode::Synchronous => {
            for arg in args {
                let split: (&str, &str) = unwrap_option_or_continue!(arg.split_once("="));
                match &split.0.to_lowercase() as &str {
                    "--sens" => {
                        accel_args.sens_multiplier = split
                            .1
                            .parse::<f64>()
                            .unwrap_or_else(|_| AccelArgs::default().sens_multiplier)
                    }
                    "--gain" => {
                        accel_args.gain = split
                            .1
                            .parse::<bool>()
                            .unwrap_or_else(|_| AccelArgs::default().gain)
                    }
                    "--gamma" => {
                        accel_args.gamma = split
                            .1
                            .parse::<f64>()
                            .unwrap_or_else(|_| AccelArgs::default().gamma)
                    }
                    "--smooth" => {
                        accel_args.cap.x = split
                            .1
                            .parse::<f64>()
                            .unwrap_or_else(|_| AccelArgs::default().smooth)
                    }
                    "--motivity" => {
                        accel_args.limit = split
                            .1
                            .parse::<f64>()
                            .unwrap_or_else(|_| AccelArgs::default().motivity)
                    }
                    "--syncspeed" => {
                        accel_args.limit = split
                            .1
                            .parse::<f64>()
                            .unwrap_or_else(|_| AccelArgs::default().sync_speed)
                    }

                    _ => {}
                }
            }
        }
        types::AccelMode::Power => {
            for arg in args {
                let split: (&str, &str) = unwrap_option_or_continue!(arg.split_once("="));
                match &split.0.to_lowercase() as &str {
                    "--sens" => {
                        accel_args.sens_multiplier = split
                            .1
                            .parse::<f64>()
                            .unwrap_or_else(|_| AccelArgs::default().sens_multiplier)
                    }
                    "--gain" => {
                        accel_args.gain = split
                            .1
                            .parse::<bool>()
                            .unwrap_or_else(|_| AccelArgs::default().gain)
                    }
                    "--scale" => {
                        accel_args.scale = split
                            .1
                            .parse::<f64>()
                            .unwrap_or_else(|_| AccelArgs::default().scale)
                    }
                    "--captype" => {
                        accel_args.cap_mode = split
                            .1
                            .parse::<CapMode>()
                            .unwrap_or_else(|_| AccelArgs::default().cap_mode)
                    }
                    "--capin" => {
                        accel_args.cap.x = split
                            .1
                            .parse::<f64>()
                            .unwrap_or_else(|_| AccelArgs::default().cap.x)
                    }
                    "--capout" => {
                        accel_args.cap.y = split
                            .1
                            .parse::<f64>()
                            .unwrap_or_else(|_| AccelArgs::default().cap.y)
                    }
                    "--exponent" => {
                        accel_args.exponent_power = split
                            .1
                            .parse::<f64>()
                            .unwrap_or_else(|_| AccelArgs::default().motivity)
                    }
                    "--offsetout" => {
                        accel_args.output_offset = split
                            .1
                            .parse::<f64>()
                            .unwrap_or_else(|_| AccelArgs::default().output_offset)
                    }

                    _ => {}
                }
            }
        }
        types::AccelMode::Motivity => {
            for arg in args {
                let split: (&str, &str) = unwrap_option_or_continue!(arg.split_once("="));
                match &split.0.to_lowercase() as &str {
                    "--sens" => {
                        accel_args.sens_multiplier = split
                            .1
                            .parse::<f64>()
                            .unwrap_or_else(|_| AccelArgs::default().sens_multiplier)
                    }
                    "--gain" => {
                        accel_args.gain = split
                            .1
                            .parse::<bool>()
                            .unwrap_or_else(|_| AccelArgs::default().gain)
                    }
                    "--growthrate" => {
                        accel_args.gamma = split
                            .1
                            .parse::<f64>()
                            .unwrap_or_else(|_| AccelArgs::default().gamma)
                    }
                    "--smooth" => {
                        accel_args.cap.x = split
                            .1
                            .parse::<f64>()
                            .unwrap_or_else(|_| AccelArgs::default().smooth)
                    }
                    "--motivity" => {
                        accel_args.limit = split
                            .1
                            .parse::<f64>()
                            .unwrap_or_else(|_| AccelArgs::default().motivity)
                    }
                    "--midpoint" => {
                        accel_args.limit = split
                            .1
                            .parse::<f64>()
                            .unwrap_or_else(|_| AccelArgs::default().sync_speed)
                    }

                    _ => {}
                }
            }
        }
        types::AccelMode::Lookup => {
            for arg in args {
                let split: (&str, &str) = unwrap_option_or_continue!(arg.split_once("="));
                match &split.0.to_lowercase() as &str {
                    "--points" => {
                        accel_args.lookup_data = parse_lookup_table(split.1)
                            .unwrap_or_else(|| AccelArgs::default().lookup_data);
                    }
                    "--applyas" => {
                        accel_args.gain = match split
                            .1
                            .parse::<PointScaling>()
                            .unwrap_or_else(|_| AccelArgs::default().point_scaling)
                        {
                            PointScaling::Sens => false,
                            PointScaling::Velocity => true,
                            _ => false,
                        }
                    }

                    _ => {}
                }
            }
        }
        types::AccelMode::Noaccel => {}
    }

    return Some(accel_args);
}

pub fn parse_lookup_table(input_string: &str) -> Option<Vec<Vec2>> {
    let mut parsed_points = vec![];
    let newlines_stripped = input_string.replace("\n", "").replace(" ", "");
    let points: Vec<&str> = newlines_stripped.trim().split(";").collect();
    for point in points {
        let xy: Vec<&str> = point.split(",").collect();
        if xy.len() == 2 {
            let x_parsed = unwrap_result_or_continue!(xy[0].trim().parse::<f64>());
            let y_parsed = unwrap_result_or_continue!(xy[1].trim().parse::<f64>());
            parsed_points.push(Vec2 {
                x: x_parsed,
                y: y_parsed,
            });
        }
    }
    if parsed_points.len() < 2 {
        return None;
    }
    return Some(parsed_points);
}

fn print_help_and_exit() {
    println!("{}", include_str!("help_command.txt"));
    std::process::exit(0);
}
