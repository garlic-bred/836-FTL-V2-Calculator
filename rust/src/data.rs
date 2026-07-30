// f32 casts match the game engine's single-precision float behavior.

// 256 for build limit / 128 for bedrock ceiling
pub const PEARL_STOP_HEIGHT: f64 = 256.0;

pub const NUM_OF_ANGLES: i32 = 4;

pub const PEARL_EYE_HEIGHT: f64 = 0.25 * (0.85_f32 as f64);

pub const EXPLOSION_HEIGHT: f64 = (0.98_f32 as f64) * (0.0625_f32 as f64);

pub const BASKET_TNT_Y: f64 = 173.875 - (0.98_f32 as f64) - 0.04;

pub const BASKET_TNT_Y_MOTION: f64 = -0.04 * 0.98;

pub const BASKET_UPACCEL_TNT: i32 = 159;
pub const BASKET_UPACCEL_TNT_Y: f64 = 169.0;
pub const ALIGNMENT_TNT_Y: f64 = 172.79375;
pub const ALIGNMENT_TNT_OFFSET: f64 = 1.8125;
pub const PEARL_Y: f64 = 256.22376922490804;
pub const PEARL_Y_MOTION: f64 = -0.03338941371413851;

pub const PEARL_HORIZONTAL_OFFSET: f64 = -((0.51_f32 as f64) - 0.51);

pub const UPACCEL_TNT_Y: f64 = 248.53626183321285;
pub const UPACCEL_TNT_LONGRANGE_Y: f64 = 250.89563683321285;
pub const MAX_UPACCEL_TNT: i32 = 31;
pub const PEARL_DRAG: f64 = 0.99_f32 as f64;
