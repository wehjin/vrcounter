use cage::Cage;

#[derive(Copy, Clone, Debug)]
pub struct ScreenMetrics {
    pub max_cage: Cage,
    pub active_cage: Cage,
    pub preferred_reading_height: f32,
    pub preferred_z_increment: f32,
}

impl ScreenMetrics {
    pub fn new(cage: Cage, preferred_reading_height: f32, preferred_z_increment: f32) -> Self {
        ScreenMetrics {
            max_cage: cage,
            active_cage: cage,
            preferred_reading_height: preferred_reading_height,
            preferred_z_increment: preferred_z_increment
        }
    }

    pub fn with_active_cage(&self, active_cage: Cage) -> Self {
        ScreenMetrics { active_cage: active_cage, ..*self }
    }

    pub fn grid_units_to_main(&self, left_right_grids: f32, bottom_top_grids: f32) -> (f32, f32) {
        let left_right_main = self.preferred_reading_height * left_right_grids;
        let bottom_top_main = self.preferred_reading_height * bottom_top_grids;
        (left_right_main, bottom_top_main)
    }
}
