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
}
