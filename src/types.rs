#[derive(Debug, Clone)]
pub struct AccelArgs {
    pub mode: AccelMode,
    pub gain: bool,

    pub input_offset: f64,
    pub output_offset: f64,
    pub acceleration: f64,
    pub decay_rate: f64,
    pub gamma: f64,
    pub motivity: f64,
    pub exponent_classic: f64,
    pub scale: f64,
    pub exponent_power: f64,
    pub limit: f64,
    pub sync_speed: f64,
    pub smooth: f64,

    pub cap: Vec2,
    pub cap_mode: CapMode,

    pub sens_multiplier: f64,
}

impl Default for AccelArgs {
    fn default() -> Self {
        AccelArgs {
            mode: AccelMode::Noaccel,
            gain: true,
            input_offset: 0.0,
            output_offset: 0.0,
            acceleration: 0.005,
            decay_rate: 0.1,
            gamma: 1.0,
            motivity: 1.5,
            exponent_classic: 2.0,
            scale: 1.0,
            exponent_power: 0.05,
            limit: 1.5,
            sync_speed: 5.0,
            smooth: 0.5,
            //{x: input_cap, y: output_cap}
            cap: Vec2 { x: 15.0, y: 1.5 },
            cap_mode: CapMode::Output,
            sens_multiplier: 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone)]
pub enum AccelMode {
    Classic,
    Jump,
    Natural,
    Synchronous,
    Power,
    Lookup,
    Noaccel,
}

#[derive(Debug, Clone)]
pub enum CapMode {
    InputOutput,
    Input,
    Output,
}

#[derive(Debug, Clone)]
pub struct FpRepRange {
    pub start: i32,
    pub stop: i32,
    pub num: i32,
}

impl FpRepRange {
    pub fn size(&self) -> i32 {
        return (self.stop - self.start) * self.num + 1;
    }
}

#[derive(Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
