use crate::data;
use crate::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct Pearl {
    pub pos: Vec3,
    pub motion: Vec3,
}

impl Pearl {
    pub fn new(pos: Vec3, motion: Vec3) -> Self {
        Pearl { pos, motion }
    }

    // Matches Python Pearl.tick():
    //   motion += (0, -0.03, 0)
    //   motion *= float32(0.99)   <- f32 precision multiply
    //   pos += motion
    pub fn tick(&mut self) {
        self.motion = self.motion.add(Vec3::new(0.0, -0.03, 0.0));
        self.motion = self.motion.multiply(data::PEARL_DECAY);
        self.pos = self.pos.add(self.motion);
    }
}

#[derive(Clone, Copy)]
pub struct Tnt {
    pub pos: Vec3,
    pub motion: Vec3,
}

impl Tnt {
    pub fn new(pos: Vec3, motion: Vec3) -> Self {
        Tnt { pos, motion }
    }

    // Matches Python Tnt.tick():
    //   motion += (0, -0.04, 0)
    //   pos += motion
    //   motion *= 0.98            <- pure f64
    pub fn tick(&mut self) {
        self.motion = self.motion.add(Vec3::new(0.0, -0.04, 0.0));
        self.pos = self.pos.add(self.motion);
        self.motion = self.motion.multiply(0.98);
    }

    // Matches Python Tnt.calculateVelocityFromExplosion()
    // exposure: already converted from float32 by caller
    // is_tnt: True  → use entityPos.y directly
    //         False → use entityPos.y + eyeHeight
    pub fn calc_velocity_from_explosion(
        self,
        entity_pos: Vec3,
        eye_height: f64,
        exposure: f64,
        is_tnt: bool,
    ) -> Vec3 {
        let explosion_pos = self.pos.add(Vec3::new(0.0, data::EXPLOSION_HEIGHT, 0.0));
        // float32(8.0) == exactly 8.0, so no precision difference
        let dist_norm = explosion_pos.distance_to(entity_pos) / 8.0;
        if dist_norm > 1.0 {
            return Vec3::zero();
        }
        let entity_y = if is_tnt { entity_pos.y } else { entity_pos.y + eye_height };
        let dir = Vec3::new(
            entity_pos.x - explosion_pos.x,
            entity_y - explosion_pos.y,
            entity_pos.z - explosion_pos.z,
        );
        let len = dir.length();
        if len == 0.0 {
            return Vec3::zero();
        }
        dir.multiply(1.0 / len).multiply((1.0 - dist_norm) * exposure)
    }
}
