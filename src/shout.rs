pub struct Shout;

impl Shout {
    pub fn create() -> Self {
        Shout {}
    }
    pub fn present() -> Shouting {
        Shouting {}
    }
}

pub struct Shouting;

impl Shouting {
    pub fn silence() {}
    pub fn is_silenced() -> bool {
        false
    }
}