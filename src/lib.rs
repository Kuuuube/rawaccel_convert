use accel_curves::classic::classic;
use accel_curves::jump::jump;
use accel_curves::natural::natural;
use accel_curves::noaccel::noaccel;
use accel_curves::power::power;
use accel_curves::synchronous::synchronous;

pub mod accel_curves;
pub mod args_parser;
pub mod convert_curve;
pub mod generate_curve;
pub mod macros;
pub mod types;
pub mod utility;
