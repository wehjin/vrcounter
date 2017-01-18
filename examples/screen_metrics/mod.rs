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

    pub fn grid_units_to_main(&self, horizontal_grids: f32, vertical_grids: f32) -> (f32, f32) {
        (self.preferred_reading_height * horizontal_grids, self.preferred_reading_height * vertical_grids)
    }

    pub fn main_units_to_grid(&self, horizontal_mains: f32, vertical_mains: f32) -> (f32, f32) {
        (horizontal_mains / self.preferred_reading_height, vertical_mains / self.preferred_reading_height)
    }
}
