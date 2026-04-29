// Constants matching Python Data.py
// Where Python uses numpy.float32, we cast to f32 first then to f64 to get
// identical IEEE 754 single-precision rounding behavior.

pub const MAX_TNT: i32 = 6688;
pub const PEARL_INITIAL_X: f64 = -119.49;
pub const PEARL_INITIAL_Z: f64 = 40.51;
pub const MAX_TICKS: i32 = 500;
pub const MAX_DISTANCE: f64 = 50.0;

// 256 for build limit / 128 for bedrock ceiling
pub const PEARL_STOP_HEIGHT: f64 = 256.0;

pub const NUM_OF_ANGLES: i32 = 4;

// PEARL_EYE_HEIGHT = 0.25 * float(float32(0.85))
pub const PEARL_EYE_HEIGHT: f64 = 0.25 * (0.85_f32 as f64);

// EXPLOSION_HEIGHT = float(float32(0.98)) * float(float32(0.0625))
pub const EXPLOSION_HEIGHT: f64 = (0.98_f32 as f64) * (0.0625_f32 as f64);

// BASKET_TNT_Y = 173.875 - float(float32(0.98)) - 0.04
pub const BASKET_TNT_Y: f64 = 173.875 - (0.98_f32 as f64) - 0.04;

// BASKET_TNT_Y_MOTION = -0.04 * 0.98  (pure f64, no float32)
pub const BASKET_TNT_Y_MOTION: f64 = -0.04 * 0.98;

pub const BASKET_UPACCEL_TNT: i32 = 159;
pub const BASKET_UPACCEL_TNT_Y: f64 = 169.0;
pub const ALIGNMENT_TNT_Y: f64 = 172.79375;
pub const ALIGNMENT_TNT_OFFSET: f64 = 1.8125;
pub const PEARL_Y: f64 = 256.22376922490804;
pub const PEARL_Y_MOTION: f64 = -0.03338941371413851;

// PEARL_HORIZONTAL_OFFSET = float(float32(0.51)) - 0.51
pub const PEARL_HORIZONTAL_OFFSET: f64 = (0.51_f32 as f64) - 0.51;

pub const UPACCEL_TNT_Y: f64 = 248.53626183321285;
pub const UPACCEL_TNT_LONGRANGE_Y: f64 = 250.89563683321285;
pub const MAX_UPACCEL_TNT: i32 = 31;
pub const MAX_VARIABLE_TNT: i32 = MAX_TNT - 22;

// Pearl motion decay = float(float32(0.99))
pub const PEARL_DECAY: f64 = 0.99_f32 as f64;
