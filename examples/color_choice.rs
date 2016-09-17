extern crate vrcounter;
extern crate cage;

use vrcounter::*;
use std::sync::Arc;

fn main() {
    let star_builder = Arc::new(|| MyStar);
    vrcounter::start(star_builder)
}
