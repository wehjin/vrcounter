use cage::{Cage};

#[derive(Copy, Clone, Debug, Default)]
pub struct Mist {
    id: u64,
    cage: Cage,
}

impl Mist {
    pub fn new(id: u64, cage: Cage) -> Self {
        Mist { id: id, cage: cage }
    }
    pub fn id(&self) -> u64 { self.id }
    pub fn contains(&self, x: f32, y: f32, z: f32) -> bool {
        self.cage.contains(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mist_contains() {
        let mist = Mist::new(1, Default::default());
        assert!(mist.contains(0.0,0.0, 0.0));
        assert!(!mist.contains(0.0, 0.0, 2.0));
        assert!(!mist.contains(0.0, 2.0, 0.0));
        assert!(!mist.contains(2.0, 0.0, 0.0));
    }
}